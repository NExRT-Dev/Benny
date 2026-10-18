# Architecture

## 1. Design Principles

An agent is a loop: observe, plan (LLM call), act (tool call), update state, repeat.

| Principle | In practice |
|---|---|
| Small core | The loop and domain traits have minimal dependencies and rarely change |
| Dependencies point inward | Everything depends on core; core depends on nothing project-specific |
| Model output is untrusted | Validate every tool call. Web pages, emails and files are data, never instructions |
| Fail closed on safety, open on convenience | If the approval gate breaks, block risky actions. If voice breaks, fall back to text |
| Every feature is removable | Disable via config or Cargo feature, no code change |
| Text first | Get a tested CLI agent working before voice or UI |

## 2. Repository Structure

### 2.1 Minimal workspace

```
jarvis/
  Cargo.toml              # [workspace], shared deps and lints
  rust-toolchain.toml     # pinned toolchain
  deny.toml               # cargo-deny policy
  config/
    default.toml          # safe defaults (committed)
    example.local.toml    # template; real local.toml is gitignored
  docs/
    security-model.md     # tool risk tiers and approval rules
    adr/                  # architecture decision records
  crates/
    core/                 # types + traits only (Message, Tool, LlmProvider)
    agent/                # orchestrator loop, state, budget, approval gate
    llm/                  # provider implementations (one to start)
    testing/              # MockLlm, FailingTool, SlowLlm, fixtures
  apps/
    cli/                  # text REPL: the composition root
```

### 2.2 Add crates only when needed

| Crate | Add when |
|---|---|
| tools | More than 2-3 tools and agent/cli feels crowded |
| memory | Persistence beyond one session (SQLite first, vectors later) |
| config | Config grows past a single file (defaults, file, env layers) |
| telemetry | Structured tracing and an audit log |
| mcp | Consume or expose tools via Model Context Protocol |
| sandbox | Shell, file-write or third-party plugin tools |
| voice | Only after text agent, tools and safety model are solid |
| evals/ | As soon as there is behavior you don't want to regress |
| apps/daemon, apps/tui | Background agent or terminal UI needed |

### 2.3 Dependency direction

```
apps/*  -->  agent  -->  core  <--  llm, tools, memory, voice, mcp
```

- `agent` depends only on `core`. It never imports `llm`, `tools` or `voice`.
- Concrete implementations are injected in `apps/*`, enabling mock-based tests and backend swaps.
- Keep `core` tiny (`serde`, `thiserror`, `async-trait`, `futures`). A bloated core triggers rebuilds across the workspace.

### 2.4 Root Cargo.toml baseline

```toml
[workspace]
resolver = "2"
members = ["crates/*", "apps/*"]

[workspace.package]
edition = "2021"
license = "MIT OR Apache-2.0"

[workspace.dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time", "sync"] }
serde = { version = "1", features = ["derive"] }
thiserror = "1"
tracing = "0.1"

[workspace.lints.clippy]
unwrap_used = "warn"
expect_used = "warn"
```

Member crates use `tokio.workspace = true` and `[lints] workspace = true`.

## 3. Core Abstractions

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, LlmError>;
    fn stream(&self, req: ChatRequest)
        -> BoxStream<'_, Result<Chunk, LlmError>>;
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn risk(&self) -> RiskLevel;               // ReadOnly | Reversible | Destructive
    fn schema(&self) -> serde_json::Value;     // generated from a schemars type
    async fn call(&self, args: serde_json::Value, ctx: &ToolCtx)
        -> Result<ToolOutput, ToolError>;
}

pub enum AgentState {
    Idle, Thinking, ExecutingTool, AwaitingApproval, Done,
}
```

Rules:
- Derive tool argument schemas from Rust structs (`serde` + `schemars`) so schema and code can't drift.
- Deserialize strictly. On failure, send the error back to the model for a bounded retry (2-3 attempts).
- Truncate tool output to a model-friendly size.
- Pass a `ToolCtx` (including a user/tenant id) to every tool from the start.
- Support streaming from day one.

## 4. Feature Tiers

A feature earns a higher tier only if the agent is useless or unsafe without it.

| Tier | Examples | If it fails | Startup behavior |
|---|---|---|---|
| Core | Agent loop, LLM provider, config | Fail fast with a clear error | Must succeed or abort |
| Important | Memory, approval gate, audit log | Degrade and warn (approval gate: fail closed) | Retry, then continue degraded |
| Optional | Voice, calendar, web search, UI, MCP plugins | Disable only that feature | Init in parallel, each with a timeout |

Safety exception: if the approval gate or permission policy can't initialize, block destructive and external tools. Never fall back to "allow everything".

## 5. Resilience

### 5.1 Questions before adding any feature
- Does the core loop need it? If not, make it a plugin or optional crate.
- What is its failure mode: crash, hang, slow, or wrong output? Hangs are the most dangerous, so use timeouts everywhere.
- Can it be disabled without a code change?
- Is there a fallback? (voice to text, local memory to in-session memory, primary LLM to secondary)
- Does it share mutable state with other features? Prefer channels and messages.
- What is its blast radius? Data corruption, secret leakage or destructive actions demand the strongest isolation.

### 5.2 Isolation levels (strongest to weakest)

| Level | Use for | Trade-off |
|---|---|---|
| Separate process | Shell tools, MCP servers, third-party code | IPC overhead; strongest containment |
| WASM sandbox (wasmtime) | Untrusted plugin tools | Limited host access; extra tooling |
| Supervised tokio task | Voice pipeline, background watchers | Panic is contained but shares the process |
| Plain Result errors | Trivial, pure features | No protection from panics or hangs |

### 5.3 Supervised restart with backoff

```rust
async fn supervise<F, Fut>(name: &str, mut make: F, cancel: CancellationToken)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
{
    let mut delay = Duration::from_secs(1);
    while !cancel.is_cancelled() {
        match tokio::spawn(make()).await {
            Ok(Ok(())) => break,
            Ok(Err(e)) => tracing::error!(feature = name, error = %e, "failed"),
            Err(j) => tracing::error!(feature = name, "panicked: {j}"),
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_secs(60));
    }
}
```

### 5.4 Feature registry with health state

```rust
pub enum Health {
    Healthy,
    Degraded(String),
    Disabled(String),
}
```

At startup: init core (abort on error), then each optional feature individually with a timeout. On failure, record `Disabled(reason)`, log it, and continue booting. Never `?`-propagate optional init failures out of `main`.

### 5.5 Other required practices
- Circuit breaker: after N consecutive failures, stop calling a flaky dependency for a cooldown period and report it unavailable.
- Tool errors are observations: return "calendar unavailable" to the model so it can adapt instead of crashing the loop.
- Prune the tool list: remove disabled tools from the schema sent to the model.
- Timeouts on every external call; bounded channels so a slow consumer can't exhaust memory.
- Cancellation: a `CancellationToken` through every long-running task.
- Budgets: cap iterations, tokens and cost per task.
- No `unwrap()`/`expect()` outside tests and startup invariants. Never hold a `std::sync::Mutex` guard across `.await`.
- Retry only retryable errors (rate limits, timeouts, 5xx) with exponential backoff and jitter.

## 10. Scaling Notes

| Scale | Assessment |
|---|---|
| Codebase / team | Scales well. Trait boundaries and crates allow independent work. Guard core against bloat |
| Tools / plugins | Scales structurally. Model tool selection degrades past roughly 20-30 tools, so add grouping or retrieval. Version the plugin interface early |
| Single-agent performance | LLM latency and rate limits are the bottleneck, not Rust. Add shared rate limiters and bounded channels for many agents |
| Multi-user hosting | Not as designed. Needs Postgres with isolation, externalized state, per-tenant sandboxes (containers/microVMs), a gateway such as axum, and per-user secrets. Passing a user/tenant id through `ToolCtx` early makes this feasible |

Where it will really hurt: context management for long-running agents, LLM cost and latency, voice latency, and evaluating agent quality. Invest in summarization/retrieval, caching and evals before micro-optimizing Rust.

## 11. Crates to Evaluate (verify before adopting)

| Area | Candidates |
|---|---|
| Async | tokio, tokio-util (CancellationToken), futures |
| Serialization/schema | serde, serde_json, schemars |
| HTTP/streaming | reqwest, eventsource-stream or reqwest-eventsource |
| LLM clients | Own thin client on reqwest, or evaluate rig, genai, async-openai, ollama-rs |
| Local inference | Ollama or llama.cpp server, llama-cpp-2, candle, ort |
| MCP | rmcp (official Rust SDK) |
| Errors/retry | thiserror, anyhow, backon or tokio-retry |
| Storage | rusqlite or sqlx; sqlite-vec, Qdrant or LanceDB for vectors |
| Config | figment or config |
| Observability | tracing, tracing-subscriber, opentelemetry |
| Secrets | secrecy, keyring |
| Sandboxing | wasmtime, OS process isolation, containers |
| Audio | cpal, rodio, whisper-rs, Silero VAD via ort, Piper for TTS |
| Quality | clippy, cargo-deny, cargo-audit, proptest |

## 12. Voice Pipeline Notes (build last)

- Pipeline: mic capture, VAD or wake word, speech-to-text, agent, text-to-speech, playback.
- Connect stages with bounded mpsc channels, not shared mutable state.
- Stream TTS sentence by sentence so speech starts before the full reply is generated.
- Barge-in: when the user speaks, cancel TTS and in-flight generation via the cancellation token.
- Keep audio on a dedicated thread; never block the audio callback.
- Run voice as a supervised, optional feature so a failure falls back to text.

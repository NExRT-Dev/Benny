# Features

## Current

- Prints `Hello, world!` on startup (`src/main.rs`).

## Feature Tiers

A feature earns a higher tier only if the agent is useless or unsafe without it.

| Tier | Examples | If it fails | Startup behavior |
|---|---|---|---|
| Core | Agent loop, LLM provider, config | Fail fast with a clear error | Must succeed or abort |
| Important | Memory, approval gate, audit log | Degrade and warn (approval gate: fail closed) | Retry, then continue degraded |
| Optional | Voice, calendar, web search, UI, MCP plugins | Disable only that feature | Init in parallel, each with a timeout |

Safety exception: if the approval gate or permission policy can't initialize, block destructive and external tools. Never fall back to "allow everything".

## Build Order (with exit criteria)

Don't start a milestone until the previous one meets its exit criteria.

| # | Milestone | Exit criteria |
|---|---|---|
| 1 | Workspace + core traits + one streaming LLM client | Build and clippy pass. A script streams a reply. Timeouts and retry on retryable errors work |
| 2 | Agent loop with iteration and budget limits | Loop runs end to end against MockLlm. Iteration cap stops cleanly. Cancellation works mid-run |
| 3 | 2-3 read-only tools with typed schemas | Schemas generated from types. Malformed model JSON triggers a bounded retry, not a crash |
| 4 | Approval / permission layer | Destructive call blocks until approved. With the gate broken, destructive tools are refused |
| 5 | Tracing + failure-mode tests | Each tool and provider tested for error, hang (timeout fires), panic, garbage output |
| 6 | Memory (SQLite) | Facts persist across sessions. Agent still runs if DB is missing/corrupt. User can inspect and delete memories |
| 7 | Write / destructive tools behind approvals | Allowlists enforced. Every call in audit log. Sandbox tested with hostile inputs |
| 8 | Feature registry + supervision | Killing an optional feature at runtime doesn't stop the agent. Disabled tools vanish from the tool list |
| 9 | Voice (optional feature flag) | Build works with voice off. Barge-in cancels speech and generation. Voice failure falls back to text |
| 10 | UI/daemon, MCP, expanded evals | Evals pass on tool choice and refusals. No regression against recorded transcripts |

## Feature Status

| Feature | Status | Notes |
|---|---|---|
| Hello world output | Done | Initial scaffold |

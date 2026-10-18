# Guidelines

## Code Style (Rust Pro Tips)

- Follow `rustfmt` formatting: run `cargo fmt` before committing (Tip 18).
- Follow `clippy` lints: run `cargo clippy -- -D warnings` (Tip 15).
- Use `cargo clippy --fix` to apply suggested fixes automatically (Tip 17).
- Use edition 2024 idioms.

## Debugging

- Use the `dbg!` macro for quick debugging; it prints the expression with file/line and returns the value (Tip 1).
- Use `todo!()` as a placeholder for unimplemented functionality; it panics with a clear message (Tip 2).

## Workflow

- Practice compiler-driven development: write code, let the compiler guide you (Tip 5).
- Run `cargo check`/`cargo build` frequently; use `bacon` for background checking (Tip 3).
- Use `cargo nextest` as a faster test runner (Tip 4).
- Run `cargo test` before pushing.
- Create a feature branch for non-trivial changes.
- Keep commits small and focused.

## Project Structure

- Keep `main.rs` thin; move logic into a library (`lib.rs`) and modules (Tip 8).
- Organize code with modules; control visibility with `pub`/`pub(crate)`/`pub(super)` (Tip 9, Tip 11).
- Create a `prelude` module re-exporting common imports (Tip 10).
- Use a Cargo workspace for multi-crate projects (Tip 12).

## Design Patterns

- Reuse types; prefer the newtype pattern for distinct domain types (Tip 6).
- Use macros (declarative/procedural) to reduce boilerplate (Tip 7).
- Use parse constructors: validate/parse at construction, return `Result` (Tip 13).
- Apply the type-state pattern to encode state in the type system (Tip 14).
- Annotate important return values with `#[must_use]` (Tip 16).

## Naming

- Use `snake_case` for functions/variables/modules.
- Use `PascalCase` for types/traits.
- Use `SCREAMING_SNAKE_CASE` for constants.

## Tooling

- Pin the toolchain via `rust-toolchain.toml` (Tip 19).
- Run `cargo audit` to check for known vulnerabilities (Tip 20).
- Use `cargo deny` for license and advisory checks (Tip 21).
- Measure coverage with `cargo tarpaulin` (Tip 22).
- Use `cargo chef` for Docker layer caching (Tip 23).

## Security Model Practices

- Use allowlists (directories, commands, domains), never denylists.
- Defend against prompt injection: content from web pages, emails and files must not silently trigger privileged tools.
- Keep secrets out of prompts and logs. Load from env or OS keychain; wrap in a secret type (e.g. `secrecy`).
- Keep an audit log of every tool call with arguments and results.
- Sandbox risky execution: separate process with dropped privileges, container, or WASM.
- Never commit secrets, local config, audit logs or memory databases.
- See `docs/security-model.md`.

## Testing and Observability

Testing layers:
- Unit tests inline with `#[cfg(test)]`.
- Crate integration tests in each crate's `tests/`.
- Cross-crate scenarios in root `tests/` using recorded LLM transcripts (record/replay).
- Failure mocks in the testing crate: `FailingTool`, `SlowLlm`, `FlakyMemory`, a panicking tool, a garbage-returning tool.
- Property tests/fuzzing (`proptest`) for parsing model output.
- Evals: scenarios checking the agent picks the right tool, refuses unsafe actions, and degrades sensibly (e.g. "calendar is down, user asks about schedule").

Observability:
- Use `tracing` with a span per task, LLM call and tool call. Record latency, token counts and errors.
- JSON log output; add OpenTelemetry only when needed.
- Surface feature health (Healthy / Degraded / Disabled) to the user.

## Verification Checklists

For every new feature:
- [ ] Tier assigned (core, important or optional)
- [ ] Behind a trait or plugin boundary
- [ ] Config flag or Cargo feature to disable it
- [ ] Timeout and retry policy defined
- [ ] Fallback or graceful "unavailable" behavior defined
- [ ] Isolation level matches blast radius
- [ ] Health reported in registry; failures logged with tracing
- [ ] Failure-mode tests written (error, hang, panic, garbage)
- [ ] Risk class assigned if it's a tool; approval policy applied

Pre-release:
- [ ] `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` pass
- [ ] `cargo deny check` and `cargo audit` are clean
- [ ] Builds with default features AND with all optional features off
- [ ] No secrets, local config or databases tracked in git
- [ ] Security model doc matches actual tool risk classes
- [ ] Killing each optional feature leaves the agent running
- [ ] Approval gate fails closed (tested)
- [ ] Evals and recorded-transcript regression tests pass
- [ ] Dependency maintenance status re-checked

## Documentation

- Document public items with `///` doc comments.
- Update `architecture.md` and `features.md` when adding modules.

## Testing

- Add unit tests alongside new code (`#[cfg(test)]`).
- Add integration tests under `tests/` when appropriate.

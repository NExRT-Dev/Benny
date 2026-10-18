# Benny

A Rust project (edition 2024) providing a minimal starting point for an agent workspace.

## Overview

Benny is a Rust crate initialized with a standard `cargo new` layout. The entry point lives in `src/main.rs`. The long-term target is a modular agent workspace (see `architecture.md`).

## Getting Started

### Prerequisites

- Rust toolchain (rustc, cargo) installed. Install via [rustup](https://rustup.rs/).

### Build

```sh
cargo build
```

### Run

```sh
cargo run
```

### Test

```sh
cargo test
```

## Project Structure

```
Benny/
|-- Cargo.toml
|-- src/
|   |-- lib.rs
|   |-- main.rs
|   |-- prelude.rs
|   |-- core/
|   |-- engine/
|   |-- plugins/
|-- docs/
|   |-- security-model.md
|   |-- adr/
├── agents.md
├── architecture.md
├── features.md
├── guidelines.md
├── readme.md
└── target/        # build artifacts (gitignored)
```

## Documentation

- `architecture.md` — design principles, structure, abstractions, resilience, scaling.
- `features.md` — feature tiers and build order.
- `guidelines.md` — code style, tooling, security, testing, verification checklists.
- `docs/security-model.md` — tool risk tiers and approval rules.

## Email (Brevo)

Transactional email is sent via the Brevo API (`POST https://api.brevo.com/v3/smtp/email`).
The API key is read from the environment variable `89hc97` (non-obvious name by design).
Set it before running the email feature:

```sh
$env:API_KEY = "your-brevo-api-key"
cargo run -- email
```

## SMS (Traccar)

SMS is sent via the Traccar SMS Gateway HTTP API (POST to the URL shown in the app).
The authorization token is read from the environment variable `7k2m9q` (non-obvious name by design).

```sh
$env:API_TOKEN = "your-traccar-token"
cargo run -- sms
```

## Plugin Engine

The crate exposes a small plugin host (`Engine`) and a `Plugin` trait.

```rust
use benny::{Context, Engine, EngineError, Plugin};

struct MyPlugin;
impl Plugin for MyPlugin {
    fn name(&self) -> &str { "my_plugin" }
    fn run(&mut self, ctx: &mut Context) -> Result<(), EngineError> {
        println!("hello from plugin");
        Ok(())
    }
}

fn main() {
    let mut engine = Engine::new();
    engine.register(MyPlugin);
    engine.init_all().unwrap();
    engine.run().unwrap();
    engine.shutdown();
}
```

To add a feature as a plugin: implement `Plugin`, then `engine.register(...)`. Disable at runtime with `engine.disable("name")`.

## License

TBD

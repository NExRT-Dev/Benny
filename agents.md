# Agents

## Purpose

This file describes how AI coding agents (e.g. opencode) should operate in this repository.

## Scope

- Repository: Benny (Rust crate).
- Working directory: project root.

## Rules for Agents

- Prefer editing existing files over creating new ones.
- Do not commit secrets or credentials.
- Do not modify `target/` (build artifacts).
- Run `cargo fmt`, `cargo clippy`, and `cargo test` after changes.
- Keep changes minimal and focused on the request.

## File Conventions

- Documentation lives as Markdown at the repo root.
- Source code lives under `src/`.

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`

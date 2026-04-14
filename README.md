# tradenal-rs

Rust backend foundation for the Tradenal ecosystem.

## Scope

This repository is the starting point for a production-grade Rust codebase that will eventually power:
- shared domain logic for trading concepts, calculations, validation, and portfolio rules
- CLI workflows for local development, admin scripts, backfills, and data tooling
- future backend services built on top of reusable core crates

## Non-goals for this bootstrap PR

This first PR intentionally does **not** include:
- HTTP APIs
- database integration
- authentication or authorization
- broker integrations
- event streaming or background jobs
- full domain modeling for trades, journals, or analytics

## Workspace layout

```text
tradenal-rs/
|-- Cargo.toml
|-- rust-toolchain.toml
|-- crates/
|   |-- trade-core/
|   `-- trade-cli/
`-- .github/workflows/ci.yml
```

## Crates

### `trade-core`
Reusable library crate for domain logic and shared business rules.

### `trade-cli`
Small CLI binary crate that depends on `trade-core` and is useful for smoke tests, local tooling, and future admin commands.

## Getting started

### Prerequisites
- Rustup installed
- Rust toolchain `1.85.0`

### Install the toolchain

```bash
rustup toolchain install 1.85.0
rustup default 1.85.0
cargo --version
```

### Workspace commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo run -p trade-cli -- healthcheck
```

## Development standards

- no `unwrap`, `expect`, `todo!`, or `panic!` in normal code paths
- shared logic belongs in `trade-core`
- binaries should stay thin and delegate to library crates
- CI must stay green for format, lint, and test checks

## Planned follow-up PRs

1. domain types for trades, journals, and money/value objects
2. config and environment loading
3. persistence layer and repository traits
4. API crate and HTTP server
5. observability, structured logging, and tracing

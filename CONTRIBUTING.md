# Contributing

## Branching

- create small, reviewable PRs
- keep bootstrap and infrastructure changes separate from domain-heavy work
- prefer one concern per PR

## Local checks

Run all checks before opening a PR:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

## Style

- prefer explicit naming over short abbreviations
- keep modules small and composable
- document public APIs
- avoid hidden magic in constructors and helpers
- return typed errors where it improves clarity

## Testing

- unit tests should live close to the code they verify
- integration tests can be added under crate-level `tests/` directories later
- every bug fix should include a regression test when practical

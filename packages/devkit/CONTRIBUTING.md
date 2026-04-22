# Contributing to stellar-devkit

## PR Requirements

- One logical change per PR
- PR title must follow `feat|fix|chore(devkit[/module]): short description`
- All issues closed by the PR must be listed in the body as `Closes #N`
- Branch off `main`; target `main`

## Test Expectations

- Every new module or function must have at least one test
- Integration tests go in `packages/devkit/tests/`
- Unit tests go in the same file as the code under `#[cfg(test)]`
- Run `cargo test -p stellar-devkit` before opening a PR

## Code Style

- Follow standard Rust formatting: `cargo fmt --package stellar-devkit`
- No `#[allow(dead_code)]` on public items
- All public items must have a doc comment (`///`)

## Boundary Rules

- Do **not** import from `packages/core` or any live-network crate
- Do **not** make real HTTP calls; use the mock harness
- Keep `[dependencies]` minimal — prefer `[dev-dependencies]` where possible

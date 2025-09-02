# Contributing to gitclean

Thanks for your interest in contributing!

## How to contribute

1. Fork the repository and create a feature branch
2. Make your changes with clear commits
3. Ensure `cargo build --release` and `cargo test` succeed
4. Open a Pull Request describing your changes and motivation

## Code style

- Prefer functional style; avoid unnecessary global state
- Keep functions small and focused
- Cover core logic with tests where reasonable
- Write clear, concise logs/messages in English

## Project layout

- Library code lives under `src/` split into modules (`patterns`, `ignore`, `scan`, `size`, `fsops`, `util`, `types`)
- CLI is in `src/main.rs` and calls the library API
- Tests live under `tests/`

## Running

```bash
cargo check
cargo test
cargo build --release
```

## Commit messages

- Use imperative tone: "Add …", "Fix …", "Refactor …"
- Reference issues when applicable: `Fixes #123`

## Reporting issues

- Use the issue template when available
- Provide steps to reproduce, expected vs actual behavior, and environment

# CLAUDE.md - key-rs Project Guidelines

## Build & Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Format: `cargo fmt`
- Lint: `cargo clippy`
- Test all: `cargo test`
- Test single: `cargo test test_name`

## Code Style Guidelines
- **Formatting**: Use `cargo fmt` before committing
- **Naming**: snake_case for variables/functions, PascalCase for types
- **Imports**: Group by crate, external crates first: `use std::{collections::HashMap, net::SocketAddr};`
- **Error handling**: Use `anyhow` for application code, add context with `.context()`
- **Concurrency**: Use `Arc<Mutex<T>>` for shared state, clone Arc references when passing to threads
- **Types**: Use `String` for owned strings, prefer strong typing over primitive types
- **Comments**: Document public APIs, add TODOs for incomplete code
- **Testing**: Write tests in a `tests` module, use descriptive test names
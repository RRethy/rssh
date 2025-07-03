# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

**Development Setup:**
- `dev up` - Sets up Rust development environment

**Build and Run:**
- `cargo build` - Build the project
- `cargo run` - Build and run the project
- `cargo build --release` - Build optimized release binary

**Code Quality:**
- `dev style` - Run formatting and linting (cargo fmt + clippy with auto-fixes)
- `cargo fmt` - Format code according to Rust standards
- `cargo clippy` - Run Rust linter for code improvements

**Testing:**
- `cargo test` - Run all tests
- `cargo test test_name` - Run specific test
- `cargo test -- --nocapture` - Run tests with stdout/stderr output

## Architecture

This is a Rust project using Cargo as the build system. The project follows standard Rust conventions:

- **src/main.rs**: Application entry point
- **Cargo.toml**: Project manifest defining dependencies and metadata
- **target/**: Build artifacts (ignored by git)

The project integrates with Shopify's `dev` tooling for consistent development workflows.

## Development Workflow

1. Use `dev up` to ensure Rust toolchain is properly configured
2. Make code changes in `src/`
3. Run `dev style` before committing to ensure consistent formatting
4. Use `cargo check` for quick compilation checks without building
5. Build with `cargo build` for development or `cargo build --release` for optimized builds
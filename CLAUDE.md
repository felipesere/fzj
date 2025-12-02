# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`fzj` is a command-line tool that provides fuzzy selection over JSON inputs. It reads JSON from stdin, presents a fuzzy-searchable interface to the user, and outputs the selected JSON object to stdout.

### Core Functionality

- **Input**: Reads JSON arrays from stdin
- **Processing**: Presents items for fuzzy selection with configurable field display
- **Output**: Writes the selected JSON object to stdout
- **Field filtering**: `--fields a,b,c` to display only specific fields during selection
- **Array extraction**: `--dig $path` to extract nested arrays before selection
- **Output filtering**: `--out x,y,z` to output only specific fields from the selected object

## Build and Development Commands

### Building
```bash
cargo build              # Debug build
cargo build --release    # Release build
```

### Running
```bash
cargo run                # Run in development mode
cargo run -- [args]      # Pass arguments to the binary
```

### Testing
```bash
cargo test               # Run all tests
cargo test <test_name>   # Run a specific test
cargo test -- --nocapture # Run tests with stdout visible
```

### Code Quality
```bash
cargo fmt                # Format code
cargo clippy             # Run linter
cargo check              # Fast compilation check without producing binary
```

## Architecture Notes

- **Edition**: Uses Rust 2024 edition (see Cargo.toml:4)
- **Current state**: Project is in early stages with minimal implementation (src/main.rs:1-3)
- **No dependencies yet**: Empty dependencies section in Cargo.toml suggests core implementation will be added

## Expected Implementation Areas

Based on the README requirements, the implementation will likely need:
1. JSON parsing and deserialization from stdin
2. Fuzzy matching/selection interface (TUI component)
3. Field extraction and filtering logic
4. Path-based nested array extraction (`--dig` functionality)
5. Selective output formatting (`--out` functionality)
6. CLI argument parsing for `--fields`, `--dig`, and `--out` options

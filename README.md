# Rust Mini Grep

A simple command-line text search tool built in Rust, following the tutorial from [The Rust Programming Language](https://doc.rust-lang.org/book/) book.

## Features

- **Case-sensitive search** (default)
- **Case-insensitive search** (via environment variable)
- **Error handling** with descriptive messages
- **Unit tests** for both search modes

## Usage

```bash
# Basic usage
cargo run -- <search_term> <file_path>

# Example: Search for "to" in poem.txt
cargo run -- to poem.txt

# Case-insensitive search
IGNORE_CASE=1 cargo run -- <search_term> <file_path>
```

## Running Tests

```bash
cargo test
```

## Project Structure

- `src/main.rs` - CLI argument parsing and main application logic
- `src/lib.rs` - Search functionality with case-sensitive and case-insensitive modes
- `poem.txt` - Sample text file for testing

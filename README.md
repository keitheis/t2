# t2: Rerun commands when files change

A fast, efficient file watcher written in Rust that automatically reruns commands when files change.

## Overview

**t2** is a Rust implementation of the file watcher. It watches files for changes and automatically reruns specified commands, making it perfect for development workflows where you need to rebuild, retest, or recompile after code changes.

## Key Features

- **Event-based watching**: Uses the `notify` crate for efficient OS-level file system events (no polling!)
- **Debouncing**: 500ms debounce to avoid running commands multiple times for rapid changes
- **Fast**: Compiled Rust binary with instant startup and minimal resource usage
- **Gaming-themed**: Fun output messages (FIGHT!, CONTINUE?, GAMEOVER)
- **Smart filtering**: Automatically ignores common temporary and binary files
- **Cross-platform**: Works on macOS, Linux, and Windows

## Installation

### From crates.io

```bash
cargo install t2
```

### From Source

```bash
cargo install --path .
```

### Build

```bash
cargo build --release
```

The binary will be at `target/release/t2`.

## Usage

```bash
t2 'command' [monitor_path [monitored_path...]]
```

### Examples

Watch Rust files and run tests:
```bash
t2 'cargo test' src/
```

Watch markdown files and rebuild docs:
```bash
t2 'make docs' *.md docs/
```

Watch multiple directories:
```bash
t2 'npm run build' src/ static/ templates/
```

## How It Works

1. **Startup**: t2 scans specified paths and runs the command immediately
2. **Watching**: Monitors files using OS-level file system events
3. **On Change**: When a file changes, prints "FIGHT!" and reruns the command
4. **Completion**: After command finishes, prints "CONTINUE?" and waits for next change
5. **Exit**: Press Ctrl+C to stop, prints "GAMEOVER"

## Ignored File Types

t2 automatically ignores changes to these file types:
- Python bytecode: `.pyc`
- Editor swap files: `.swp`, `.swo`
- Images: `.bmp`, `.jpg`, `.jpeg`, `.png`, `.gif`, `.svg`, `.psd`, `.xcf`, `.pxm`

## Improvements Over p2 (Python Version)

- **Event-based watching** instead of 1-second polling (faster, more efficient)
- **Instant response** to file changes
- **Lower CPU usage** (no constant polling loop)
- **Faster startup** (compiled binary vs interpreted Python)
- **Type safety** and memory safety from Rust
- **Debouncing** to handle rapid file changes gracefully

## Output Example

```
$ t2 'echo "Building..."' src/
t2 is watching about 8 files:
src/
FIGHT!
echo "Building..."
Building...
CONTINUE?
src/main.rs changed
FIGHT!
echo "Building..."
Building...
CONTINUE?
^CGAMEOVER
```

## Development

### Run Tests

```bash
cargo test
```

### Build for Development

```bash
cargo build
```

### Build for Release

```bash
cargo build --release
```

## Project Structure

```
t2/
├── Cargo.toml              # Project configuration
├── src/
│   ├── main.rs             # Entry point and orchestration
│   ├── filter.rs           # File ignore patterns
│   ├── scanner.rs          # File discovery
│   ├── executor.rs         # Command execution
│   ├── watcher.rs          # Event-based file watching
└── tests/
    └── integration_tests.rs # Integration tests
```

## License

MIT License

## Author

Keith Yang <yang@keitheis.org>

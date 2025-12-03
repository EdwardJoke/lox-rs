# lox-rs

[![uv](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/uv/main/assets/badge/v0.json)](https://github.com/astral-sh/uv) [![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/EdwardJoke/lox-rs/blob/main/LICENSE) [![Release](https://img.shields.io/github/v/release/EdwardJoke/lox-rs)](https://github.com/EdwardJoke/lox-rs/releases)

A command-line interface tool that makes executing your project commands easier, supporting both Rust and Python projects.

## Overview

lox-rs is a CLI tool designed to simplify common project workflows. It provides a unified interface for building, running, and checking your Rust or uv projects with consistent output formatting.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/EdwardJoke/lox-rs.git
cd lox-rs

# Build and install
cargo install --path .

# Or use as a local tool
cargo build --release
target/release/lox --help
```

### Usage as a Local Tool

If you prefer to use lox-rs as a local tool without installing it globally:

```bash
# Run directly with cargo
lox --help
```

## Commands

### `lox doctor`

Check the project information and environment details, automatically detecting project type.

**Usage:**
```bash
lox doctor
```

**Example Output (Rust):**
```
[TIP] + Never run the doctor command in the project before.

[1/2] + Project informations
  - Project type:           app(bin) (rust)
  - Project name:           lox
  - Project version:        0.1.0
  - Project build(dev):     cargo build
  - Project build(release): cargo build --release
  - Project fmt:            cargo fmt
  - Project lint:           cargo check
  - Project dependency:     cargo update

[2/2] + Environment informations
  - Operating system:      macOS
  - CPU architecture:      x86_64
  - RustC version:         1.91.1
  - Cargo version:         1.91.1

[TIP] + Project configuration saved to `lox.toml`.
[TIP] + Everything is Up-to-date.
[TIP] + [Task End]
```

**Example Output (Python | uv):**
```
[TIP] + Never run the doctor command in the project before.

[1/2] + Project informations
  - Project type:           uv (python)
  - Project name:           uv
  - Project version:        0.1.0
  - Project virtual env:    unknown
  - Project build:          uv build
  - Project fmt:            uvx ruff format
  - Project lint:           uvx ruff check
  - Project dependency:     uv update

[2/2] + Environment informations
  - Operating system:      macOS
  - CPU architecture:      x86_64
  - uv version:            0.9.13

[TIP] + Project configuration saved to `lox.toml`.
[TIP] + Everything is Up-to-date.
[TIP] + [Task End]
```

### `lox dev`

Build the project in development mode.

**Usage:**
```bash
lox dev
```

**Example Output:**
```
[TIP] + Build for Dev.

[1/3] + Download dependencies
  - Task | cargo update | Done.
  - Task | cargo fmt    | Done.

[2/3] + Check the project
  - Task | cargo check  | Done.

[3/3] + Build the project
  - Task | cargo build  | Done.

[TIP] + Build at + `./target/debug/lox-rs` .
[TIP] + [Task End]
```

### `lox build`

Build the project in release mode, with different behaviors for Rust and Python projects.

**Usage:**
```bash
lox build
```

**Example Output (Rust):**
```
[TIP] + Build for Release.

[1/3] + Download dependencies
  - Task | cargo update | Done.
  - Task | cargo fmt    | Done.

[2/3] + Check the project
  - Task | cargo check  | Done.

[3/3] + Build the project
  - Task | cargo build --release | Done.

[TIP] + Build at + `./target/release/lox` .
[TIP] + [Task End]
```

**Example Output (Python | uv):**
```
[TIP] + Build the project.

[1/3] + Lock the project dependencies
  - Task | uv lock         | Done.

[2/3] + Check and Format the project
  - Task | uvx ruff check  | Done.
  - Task | uvx ruff format | Done.

[3/3] + Build the project
  - Task | uv build        | Done.

[TIP] + Build at + `dist` .
[TIP] + [Task End]
```

### `lox dash`

Run the project in development mode (builds first if needed).

**Usage:**
```bash
lox dash
```

**Example Output (Rust):**
```
[TIP] + Nothing at `target` .

[1/2] + Build the project first.
  - Task | lox dev | Done.

[2/2] + Run the project.
  - Task | ./target/debug/lox | Done.

[TIP] + Run the project in 0.85s.
[TIP] + [Task End]
```

### `lox run`

Run the project in release mode (builds first if needed), with different behaviors for Rust and Python projects.

**Usage:**
```bash
lox run
```

**Example Output (Rust):**
```
[TIP] + Nothing at `target` .

[1/2] + Build the project first.
  - Task | lox build | Done.

[2/2] + Run the project.
  - Task | ./target/release/lox | Done.

[TIP] + Run the project in 0.76s.
[TIP] + [Task End]
```

**Example Output (Python | uv):**
```
[1/2] + Lock the project dependencies.
  - Task | uv lock | Done.

[2/2] + Run the project.
  - Task | uv run main.py | Done.

[TIP] + Run the project in 0.92s.
[TIP] + [Task End]
```

## Features

- Consistent output formatting across all commands
- Automatic detection of project configuration
- First-run detection and messaging
- Smart build checking (only builds when needed)
- Comprehensive environment information

## Project Structure

```
lox-rs/
├── src/
│   ├── main.rs          # Main CLI entry point
│   └── commands/        # Command implementations
│       ├── mod.rs       # Command module exports
│       ├── doctor.rs    # doctor command implementation
│       ├── dev.rs       # dev command implementation
│       ├── build.rs     # build command implementation
│       ├── dash.rs      # dash command implementation
│       └── run.rs       # run command implementation
├── Cargo.toml           # Project configuration
└── README.md            # This file
```

## License

MIT

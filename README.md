# lox-rs

A command-line interface tool that makes executing your project commands easier (Now just support Rust).

## Overview

lox-rs is a CLI tool designed to simplify common project workflows. It provides a unified interface for building, running, and checking your Rust projects with consistent output formatting.

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

Check the project information and environment details.

**Usage:**
```bash
lox doctor
```

**Example Output:**
```
[TIP] + Never run the doctor command in the project before.

[1/2] + Project informations
  - Project type:           app(bin)
  - Project name:           lox-rs
  - Project version:        0.1.0
  - Project build(dev):     cargo build
  - Project build(release): cargo build --release

[2/2] + Environment informations
  - Operating system:      macOS
  - CPU architecture:      x86_64
  - RustC version:         1.91.1
  - Cargo version:         1.91.1

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

Build the project in release mode.

**Usage:**
```bash
lox build
```

**Example Output:**
```
[TIP] + Build for Release.

[1/3] + Download dependencies
  - Task | cargo update | Done.
  - Task | cargo fmt    | Done.

[2/3] + Check the project
  - Task | cargo check  | Done.

[3/3] + Build the project
  - Task | cargo build --release | Done.

[TIP] + Build at + `./target/release/lox-rs` .
[TIP] + [Task End]
```

### `lox dash`

Run the project in development mode (builds first if needed).

**Usage:**
```bash
lox dash
```

**Example Output:**
```
[TIP] + Nothing at `target` .

[1/2] + Build the project first.
  - Task | lox dev | Done.

[2/2] + Run the project.
  - Task | ./target/debug/lox-rs | Done.

[TIP] + Run the project in 0.56s.
[TIP] + [Task End]
```

### `lox run`

Run the project in release mode (builds first if needed).

**Usage:**
```bash
lox run
```

**Example Output:**
```
[TIP] + Nothing at `target` .

[1/2] + Build the project first.
  - Task | lox build | Done.

[2/2] + Run the project.
  - Task | ./target/release/lox-rs | Done.

[TIP] + Run the project in 0.56s.
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
│   ├── doctor.rs        # doctor command implementation
│   ├── dev.rs           # dev command implementation
│   ├── build.rs         # build command implementation
│   ├── dash.rs          # dash command implementation
│   └── run.rs           # run command implementation
├── Cargo.toml           # Project configuration
└── README.md            # This file
```

## License

MIT

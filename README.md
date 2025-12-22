# lox-rs

[![uv](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/uv/main/assets/badge/v0.json)](https://github.com/astral-sh/uv) [![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/EdwardJoke/lox-rs/blob/main/LICENSE) [![Release](https://img.shields.io/github/v/release/EdwardJoke/lox-rs)](https://github.com/EdwardJoke/lox-rs/releases) [![Build](https://github.com/EdwardJoke/lox-rs/actions/workflows/rust.yml/badge.svg)]

A command-line interface tool that makes managing your project commands easier, supporting Rust, Fortran (`fpm`, `built-in native`) and Python (`uv`) projects.

## Overview

lox-rs is a CLI tool designed to simplify common project workflows. It provides a unified interface for building, running, and checking your Rust or uv projects with consistent output formatting.

## Demo

![demo](assets/demo/demo.gif)

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

## Features

- Consistent output formatting across all commands
- Automatic detection of project configuration
- First-run detection and messaging
- Smart build checking (only builds when needed)
- Comprehensive environment information

## Q&A

- **Q: I am using the official pip as my environment configuration tool/package manager. Am I unable to use this tool?**
  - **A: Yes, we apologize that `lox-rs` is temporarily unavailable, but we recommend using `uv`, a fast, high-performance package manager written in `Rust` who can do the same thing as `pip`. For more details, please visit the [official `Astral` website](https://astral.sh/).** The `pip` support is planned for the future (v0.4).

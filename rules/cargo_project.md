# Cargo project rules

## app(bin) / library(lib) | Started at v0.1.1
The detection method for `cargo` projects is whether a `Cargo.toml` file exists. Therefore, please ensure it is present in your project.
### File tree
```
# cargo project
[cargo-example]
├── Cargo.toml
├── .gitignore
└── src
    └── main.rs
```
### Command
The cargo projects supports the `dash`(debug), `run` and `dev`(debug), `build` commands.
### Build
To build the cargo project, run the following command:
```bash
lox dev   # Build the project in debug mode
lox build # Build the project in release mode
```
The output files are in the `target` directory with the target files (Only app(bin) will have the executable files):
app(bin):
- `cargo-example` # MacOS or Linux
- `cargo-example.exe` # Windows
library(lib):
- `cargo_example.rlib`
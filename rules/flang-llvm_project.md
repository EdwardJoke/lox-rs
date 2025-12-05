# Fortran project rules

## LLVM Flang | Started at v0.3.0d1
Fortran project is built-in lox-rs. So, the detection method for Fortran projects is whether a `lox.toml` file exists. Therefore, please ensure it is present in your project.
### File tree
```
# Fortran project
[fortran-example]
├── lox.toml
└── src
    └── main.f90
```
### Command
The Fortran projects supports the `dash`(debug), `run` and `dev`(debug), `build` commands.
### Build
To build the Fortran project, run the following command:
```bash
lox dev   # Build the project in debug mode, output at `target/dev`
lox build # Build the project in release mode, output at `target/release`
```
The output files are in the `target` directory with two files:
- `fortran-example` # MacOS or Linux
- `fortran-example.exe` # Windows
and
- `fortran-example.out` # Cross-platform file
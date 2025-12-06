# Fpm project rules

## app(bin) / library(lib) | Started at v0.1.1
The detection method for `fpm` projects is whether a `fpm.toml` file exists. Therefore, please ensure it is present in your project.
### File tree
```
# fpm project
[fpm-example]
├── fpm.toml
├── README.md
├── app
    └── main.f90
├── test
    └── check.f90
└── src
    └── fpm.f90
```
### Command
The fpm projects supports the `dash`(debug), `run` and `dev`(debug), `build` commands.
### Build
To build the fpm project, run the following command:
```bash
lox dev   # Build the project in debug mode
lox build # Build the project in release mode
```
The output files are in the `build` directory with the target files
- `fpm-example`

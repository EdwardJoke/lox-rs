# Python project rules

## uv | Started at v0.2.1a1
The detection method for `uv` projects is whether a `pyproject.toml` file exists. Therefore, please ensure it is present in your project.
### File tree
```
# uv project
[uv-example]
├── pyproject.toml
├── .python-version
├── README.md
├── main.py
└── .venv
```
### Command
The uv project only supports the `run` and `build` commands.
### Build
To build the uv project, run the following command:
```bash
lox build
```
The output files are in the `dist` directory with two files:
- `uv-example-0.1.0.tar.gz`
- `uv-example-0.1.0-py3-none-any.whl`

## Anaconda | Planned
Still in planning.
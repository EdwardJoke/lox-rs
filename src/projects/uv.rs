use super::{BuildCommands, Project, RunCommands};
use std::fs::{metadata, read_to_string};

pub fn detect_uv_project() -> Option<Project> {
    // Check if it's a Python project (has pyproject.toml)
    if !metadata("pyproject.toml").is_ok() {
        return None;
    }

    let project_type = String::from("uv");
    let mut project_name = String::from("unknown");
    let mut project_version = String::from("unknown");
    let is_library = false;

    if let Ok(pyproject_content) = read_to_string("pyproject.toml") {
        for line in pyproject_content.lines() {
            if line.starts_with("name = ") {
                if let Some((_, rest)) = line.split_once('"') {
                    if let Some((name, _)) = rest.split_once('"') {
                        project_name = name.to_string();
                    }
                }
            } else if line.starts_with("version = ") {
                if let Some((_, rest)) = line.split_once('"') {
                    if let Some((version, _)) = rest.split_once('"') {
                        project_version = version.to_string();
                    }
                }
            }
        }
    }

    // Determine build commands for Python projects
    let build_commands = BuildCommands {
        dev: String::from("uv build"),
        release: String::from("uv build"),
    };

    // Determine run commands for Python projects
    let run_commands = RunCommands {
        dev: String::from("uv run main.py"),
        release: String::from("uv run main.py"),
    };

    Some(Project {
        project_type,
        name: project_name,
        version: project_version,
        is_library,
        build_commands,
        run_commands,
        is_rust_project: false,
        is_uv_project: true,
        is_fortran_project: false,
    })
}

use crate::projects::cargo::detect_cargo_project;
use crate::projects::flang::detect_fortran_project;
use crate::projects::fpm::detect_fpm_project;
use crate::projects::uv::detect_uv_project;
use crate::projects::{BuildCommands, Project, RunCommands, write_project_to_toml};
use tokio::fs::{metadata, read_to_string};

pub async fn get_or_create_project() -> Project {
    // Check if lox.toml exists
    let lox_toml_exists = metadata("lox.toml").await.is_ok();

    if lox_toml_exists {
        // Read and parse lox.toml
        if let Ok(mut project) = read_project_from_toml().await {
            // If run commands are unknown, detect them dynamically
            if project.run_commands.dev == "unknown" || project.run_commands.release == "unknown" {
                let detected_project = detect_project_info().await;
                project.run_commands = detected_project.run_commands;
                write_project_to_toml(&project);
            }
            return project;
        }
    }

    // If lox.toml doesn't exist or can't be parsed, create it
    let project = detect_project_info().await;
    write_project_to_toml(&project);
    project
}

pub async fn read_project_from_toml() -> Result<Project, String> {
    let toml_content = read_to_string("lox.toml")
        .await
        .map_err(|e| e.to_string())?;

    let mut project_type = String::from("unknown");
    let mut name = String::from("unknown");
    let mut version = String::from("unknown");
    let mut is_library = false;
    let mut build_dev = String::from("unknown");
    let mut build_release = String::from("unknown");
    let mut run_dev = String::from("unknown");
    let mut run_release = String::from("unknown");
    let mut is_rust_project = false;
    let mut is_uv_project = false;
    let mut is_fortran_project = false;

    let mut current_section = String::new();

    for line in toml_content.lines() {
        let trimmed_line = line.trim();

        // Skip empty lines and comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Handle section headers
        if trimmed_line.starts_with('[') && trimmed_line.ends_with(']') {
            current_section = trimmed_line[1..trimmed_line.len() - 1].trim().to_string();
            continue;
        }

        // Parse key-value pairs
        if let Some((key, value)) = trimmed_line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match current_section.as_str() {
                "project" => match key {
                    "type" => {
                        project_type = value.to_string();
                        is_library = project_type.contains("library");
                        is_rust_project =
                            project_type.contains("app") || project_type.contains("library");
                        is_uv_project = project_type == "uv";
                        is_fortran_project = project_type == "llvm-f" || project_type == "fpm";
                    }
                    "name" => name = value.to_string(),
                    "version" => version = value.to_string(),
                    _ => {}
                },
                "project.build" => match key {
                    "dev" => build_dev = value.to_string(),
                    "release" => build_release = value.to_string(),
                    _ => {}
                },
                "project.run" => match key {
                    "dev" => run_dev = value.to_string(),
                    "release" => run_release = value.to_string(),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(Project {
        project_type,
        name,
        version,
        is_library,
        build_commands: BuildCommands {
            dev: build_dev,
            release: build_release,
        },
        run_commands: RunCommands {
            dev: run_dev,
            release: run_release,
        },
        is_rust_project,
        is_uv_project,
        is_fortran_project,
    })
}

pub async fn detect_project_info() -> Project {
    // Try to detect Cargo project first
    if let Some(cargo_project) = detect_cargo_project().await {
        return cargo_project;
    }

    // Try to detect UV project next
    if let Some(uv_project) = detect_uv_project().await {
        return uv_project;
    }

    // Try to detect FPM project
    if let Some(fpm_project) = detect_fpm_project().await {
        return fpm_project;
    }

    // Try to detect Fortran project
    if let Some(fortran_project) = detect_fortran_project().await {
        return fortran_project;
    }

    // Default to unknown project type
    Project {
        project_type: String::from("unknown"),
        name: String::from("unknown"),
        version: String::from("unknown"),
        is_library: false,
        build_commands: BuildCommands {
            dev: String::from("unknown"),
            release: String::from("unknown"),
        },
        run_commands: RunCommands {
            dev: String::from("unknown"),
            release: String::from("unknown"),
        },
        is_rust_project: false,
        is_uv_project: false,
        is_fortran_project: false,
    }
}

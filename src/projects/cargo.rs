use super::{BuildCommands, Project, RunCommands};
use tokio::fs::{metadata, read_to_string};

pub async fn detect_cargo_project() -> Option<Project> {
    // Check if it's a Rust project (has Cargo.toml)
    if !metadata("Cargo.toml").await.is_ok() {
        return None;
    }

    let mut project_type = String::from("unknown");
    let mut project_name = String::from("unknown");
    let mut project_version = String::from("unknown");
    let mut is_library = false;

    if let Ok(cargo_content) = read_to_string("Cargo.toml").await {
        for line in cargo_content.lines() {
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
            } else if line.starts_with("[lib]") {
                project_type = String::from("library(lib)");
                is_library = true;
            } else if line.starts_with("[[bin]]") {
                project_type = String::from("app(bin)");
            }
        }
    }

    // If no explicit type found, default to binary if there's a main.rs
    if project_type == "unknown" {
        project_type = if metadata("src/main.rs").await.is_ok() {
            String::from("app(bin)")
        } else if metadata("src/lib.rs").await.is_ok() {
            String::from("library(lib)")
        } else {
            String::from("unknown")
        };
        is_library = project_type == "library(lib)";
    }

    // Determine build commands for Rust projects
    let build_commands = BuildCommands {
        dev: String::from("cargo build"),
        release: String::from("cargo build --release"),
    };

    // Determine run commands based on project type and binary name
    let binary_name = project_name.replace('-', "_");
    let run_commands = if !is_library {
        RunCommands {
            dev: format!("./target/debug/{}", binary_name),
            release: format!("./target/release/{}", binary_name),
        }
    } else {
        RunCommands {
            dev: String::from("unknown"),
            release: String::from("unknown"),
        }
    };

    Some(Project {
        project_type,
        name: project_name,
        version: project_version,
        is_library,
        build_commands,
        run_commands,
        is_rust_project: true,
        is_uv_project: false,
        is_fortran_project: false,
    })
}

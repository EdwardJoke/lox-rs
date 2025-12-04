use std::fs::{metadata, read_to_string, write};

#[derive(Debug)]
pub struct Project {
    pub project_type: String,
    pub name: String,
    pub version: String,
    pub is_library: bool,
    pub build_commands: BuildCommands,
    pub run_commands: RunCommands,
    pub is_rust_project: bool,
    pub is_uv_project: bool,
}

#[derive(Debug)]
pub struct BuildCommands {
    pub dev: String,
    pub release: String,
}

#[derive(Debug)]
pub struct RunCommands {
    pub dev: String,
    pub release: String,
}

pub fn get_or_create_project() -> Project {
    // Check if lox.toml exists
    let lox_toml_exists = metadata("lox.toml").is_ok();
    
    if lox_toml_exists {
        // Read and parse lox.toml
        if let Ok(mut project) = read_project_from_toml() {
            // If run commands are unknown, detect them dynamically
            if project.run_commands.dev == "unknown" || project.run_commands.release == "unknown" {
                let detected_project = detect_project_info();
                project.run_commands = detected_project.run_commands;
                write_project_to_toml(&project);
            }
            return project;
        }
    }
    
    // If lox.toml doesn't exist or can't be parsed, create it
    let project = detect_project_info();
    write_project_to_toml(&project);
    project
}

pub fn read_project_from_toml() -> Result<Project, String> {
    let toml_content = read_to_string("lox.toml").map_err(|e| e.to_string())?;

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
    })
}

pub fn detect_project_info() -> Project {
    // Check if it's a Rust project (has Cargo.toml)
    let is_rust_project = metadata("Cargo.toml").is_ok();

    // Check if it's a Python project (has pyproject.toml)
    let is_uv_project = metadata("pyproject.toml").is_ok();

    let mut project_type = String::from("unknown");
    let mut project_name = String::from("unknown");
    let mut project_version = String::from("unknown");
    let mut is_library = false;

    // Parse project info based on project type
    if is_rust_project {
        if let Ok(cargo_content) = read_to_string("Cargo.toml") {
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
            project_type = if metadata("src/main.rs").is_ok() {
                String::from("app(bin)")
            } else if metadata("src/lib.rs").is_ok() {
                String::from("library(lib)")
            } else {
                String::from("unknown")
            };
            is_library = project_type == "library(lib)";
        }
    } else if is_uv_project {
        project_type = String::from("uv");

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
    }

    // Determine build and run commands based on project type
    let build_commands = if is_rust_project {
        BuildCommands {
            dev: String::from("cargo build"),
            release: String::from("cargo build --release"),
        }
    } else if is_uv_project {
        BuildCommands {
            dev: String::from("uv build"),
            release: String::from("uv build"),
        }
    } else {
        BuildCommands {
            dev: String::from("unknown"),
            release: String::from("unknown"),
        }
    };

    // Determine run commands based on project type and binary name
    let binary_name = project_name.replace('-', "_");
    let run_commands = if is_rust_project && !is_library {
        RunCommands {
            dev: format!("./target/debug/{}", binary_name),
            release: format!("./target/release/{}", binary_name),
        }
    } else if is_uv_project {
        RunCommands {
            dev: String::from("uv run main.py"),
            release: String::from("uv run main.py"),
        }
    } else {
        RunCommands {
            dev: String::from("unknown"),
            release: String::from("unknown"),
        }
    };

    Project {
        project_type,
        name: project_name,
        version: project_version,
        is_library,
        build_commands,
        run_commands,
        is_rust_project,
        is_uv_project,
    }
}

pub fn write_project_to_toml(project: &Project) {
    // Create TOML content for project configuration
    let mut toml_content = format!(
        "[project]\ntype = \"{}\"\nname = \"{}\"\nversion = \"{}\"\n",
        project.project_type, project.name, project.version
    );

    // Add build commands
    toml_content.push_str("\n[project.build]\n");
    toml_content.push_str(format!("dev = \"{}\"\n", project.build_commands.dev).as_str());
    toml_content.push_str(format!("release = \"{}\"\n", project.build_commands.release).as_str());

    // Add run commands
    toml_content.push_str("\n[project.run]\n");
    toml_content.push_str(format!("dev = \"{}\"\n", project.run_commands.dev).as_str());
    toml_content.push_str(format!("release = \"{}\"\n", project.run_commands.release).as_str());

    // Write to lox.toml
    if let Err(e) = write("lox.toml", toml_content) {
        eprintln!(
            "Warning: Failed to write project configuration to lox.toml: {}",
            e
        );
    }
}

pub fn format_os_name(os: &str) -> String {
    match os {
        "macos" => "macOS".to_string(),
        "linux" => "Linux".to_string(),
        "windows" => "Windows".to_string(),
        _ => os.to_string(),
    }
}

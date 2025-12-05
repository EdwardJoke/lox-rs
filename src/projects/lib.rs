use std::fs::write;

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
    pub is_fortran_project: bool,
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

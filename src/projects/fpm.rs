use crate::projects::{BuildCommands, Project, RunCommands};
use tokio::fs::{metadata, read_to_string};

/// Detect FPM project information
pub async fn detect_fpm_project() -> Option<Project> {
    // Check if fpm.toml exists
    if !metadata("fpm.toml").await.is_ok() {
        return None;
    }

    // Read fpm.toml to get project name
    let mut project_name = "unknown".to_string();
    if let Ok(content) = read_to_string("fpm.toml").await {
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                if key == "name" {
                    project_name = value.to_string();
                    break;
                }
            }
        }
    }

    // Create project structure
    Some(Project {
        project_type: "fpm".to_string(),
        name: project_name,
        version: "0.1.0".to_string(),
        is_library: false,
        build_commands: BuildCommands {
            dev: "fpm build".to_string(),
            release: "fpm build --profile release".to_string(),
        },
        run_commands: RunCommands {
            dev: "fpm run".to_string(),
            release: "fpm run --profile release".to_string(),
        },
        is_rust_project: false,
        is_uv_project: false,
        is_fortran_project: true,
    })
}

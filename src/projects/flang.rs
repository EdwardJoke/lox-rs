use crate::projects::{BuildCommands, Project, RunCommands};
use std::path::Path;
use tokio::fs::{read_dir, read_to_string};

/// Detect Fortran project information
pub async fn detect_fortran_project() -> Option<Project> {
    // Check if there are any .f90 or .f files in the current directory
    let has_fortran_files = {
        let mut found = false;
        if let Ok(mut entries) = read_dir(".").await {
            while let Some(entry_result) = entries.next_entry().await.ok() {
                if let Some(entry) = entry_result {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            let ext = extension.to_str().unwrap_or("");
                            if ext == "f90" || ext == "f" || ext == "F90" || ext == "F" {
                                found = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        found
    };

    if !has_fortran_files {
        return None;
    }

    // Detect main program file (look for program keyword)
    let main_file = find_main_program_file().await?;
    let main_file_name = main_file.file_stem()?.to_str()?.to_string();

    // Create project structure
    Some(Project {
        project_type: "llvm-f".to_string(),
        name: main_file_name.clone(),
        version: "0.1.0".to_string(),
        is_library: false,
        build_commands: BuildCommands {
            dev: format!(
                "mkdir -p target/dev && flang -g -o target/dev/{0}.out {0}.f90",
                main_file_name
            ),
            release: format!(
                "mkdir -p target/release && flang -O3 -o target/release/{0}.out {0}.f90",
                main_file_name
            ),
        },
        run_commands: RunCommands {
            dev: format!("./target/dev/{}.out", main_file_name),
            release: format!("./target/release/{}.out", main_file_name),
        },
        is_rust_project: false,
        is_uv_project: false,
        is_fortran_project: true,
    })
}

/// Find the main program file by looking for the "program" keyword at the beginning of a line
pub async fn find_main_program_file() -> Option<std::path::PathBuf> {
    if let Ok(mut entries) = read_dir(".").await {
        while let Some(entry_result) = entries.next_entry().await.ok() {
            if let Some(entry) = entry_result {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_str().unwrap_or("");
                        if ext == "f90" || ext == "f" || ext == "F90" || ext == "F" {
                            if let Ok(content) = read_to_string(&path).await {
                                // Check each line for program declaration at the beginning
                                for line in content.lines() {
                                    // Trim whitespace and convert to lowercase
                                    let trimmed = line.trim().to_lowercase();
                                    // Check if line starts with "program " (actual program declaration)
                                    if trimmed.starts_with("program ") {
                                        return Some(path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Detect dependencies between Fortran files
pub async fn detect_fortran_dependencies() -> Vec<(String, Vec<String>)> {
    let mut dependencies = Vec::new();

    // Iterate over all Fortran files in the directory
    if let Ok(mut entries) = read_dir(".").await {
        while let Some(entry_result) = entries.next_entry().await.ok() {
            if let Some(entry) = entry_result {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_str().unwrap_or("");
                        if ext == "f90" || ext == "f" || ext == "F90" || ext == "F" {
                            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                            let dep_files = find_file_dependencies(&path).await;
                            dependencies.push((file_name, dep_files));
                        }
                    }
                }
            }
        }
    }

    dependencies
}

/// Find dependencies for a specific Fortran file
async fn find_file_dependencies(file_path: &Path) -> Vec<String> {
    let mut dependencies = Vec::new();

    if let Ok(content) = read_to_string(file_path).await {
        let lines = content.lines();

        // Check for use statements and include statements
        for line in lines {
            let line_lower = line.trim().to_lowercase();

            // Check for use statements (module dependencies)
            if line_lower.starts_with("use ") {
                let parts: Vec<&str> = line_lower.split_whitespace().collect();
                if parts.len() >= 2 {
                    let module_name = parts[1].to_string();
                    // Try to find the corresponding module file
                    if let Some(module_file) = find_module_file(&module_name).await {
                        dependencies.push(module_file);
                    }
                }
            }

            // Check for include statements
            if line_lower.starts_with("include ") {
                let mut include_path = line_lower.trim_start_matches("include ").trim();
                // Remove quotes if present
                if include_path.starts_with('"') && include_path.ends_with('"') {
                    include_path = &include_path[1..include_path.len() - 1];
                } else if include_path.starts_with('\'') && include_path.ends_with('\'') {
                    include_path = &include_path[1..include_path.len() - 1];
                }
                dependencies.push(include_path.to_string());
            }
        }
    }

    dependencies
}

/// Find the file that contains a specific module
async fn find_module_file(module_name: &str) -> Option<String> {
    if let Ok(mut entries) = read_dir(".").await {
        while let Some(entry_result) = entries.next_entry().await.ok() {
            if let Some(entry) = entry_result {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_str().unwrap_or("");
                        if ext == "f90" || ext == "f" || ext == "F90" || ext == "F" {
                            if let Ok(content) = read_to_string(&path).await {
                                if content
                                    .to_lowercase()
                                    .contains(&format!("module {}", module_name))
                                {
                                    return Some(
                                        path.file_name().unwrap().to_str().unwrap().to_string(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Build dependency graph and determine compilation order
pub async fn get_compilation_order() -> Vec<String> {
    let dependencies = detect_fortran_dependencies().await;
    let mut order = Vec::new();
    let mut visited = Vec::new();

    // Perform topological sort to determine compilation order
    for (file, _) in &dependencies {
        if !visited.contains(file) {
            visit(file, &dependencies, &mut visited, &mut order);
        }
    }

    order
}

/// Helper function for topological sort
fn visit(
    file: &str,
    dependencies: &Vec<(String, Vec<String>)>,
    visited: &mut Vec<String>,
    order: &mut Vec<String>,
) {
    visited.push(file.to_string());

    // Visit all dependencies first
    if let Some((_, deps)) = dependencies.iter().find(|(f, _)| f == file) {
        for dep in deps {
            if !visited.contains(dep) {
                visit(dep, dependencies, visited, order);
            }
        }
    }

    // Add file to order after all dependencies are processed
    order.push(file.to_string());
}

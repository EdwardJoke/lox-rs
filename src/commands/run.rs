use std::fs::metadata;
use std::fs::read_to_string;
use std::process::Command;
use std::time::Instant;

pub fn run() {
    println!();

    // Check if it's a Rust project (has Cargo.toml)
    let is_rust_project = metadata("Cargo.toml").is_ok();

    // Check if it's a Python project (has pyproject.toml)
    let is_uv_project = metadata("pyproject.toml").is_ok();

    if is_rust_project {
        // Check if it's a library project
        if is_library_project() {
            println!("[TIP] + The current project is a library(lib) project, which doesn't have binary output.");
            println!("[TIP] + [Task End]");
            println!();
            return;
        }
        run_rust_project();
    } else if is_uv_project {
        run_uv_project();
    } else {
        println!("[TIP] + Unknown project type. No run configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

fn is_library_project() -> bool {
    if let Ok(cargo_content) = read_to_string("Cargo.toml") {
        for line in cargo_content.lines() {
            if line.starts_with("[lib]") {
                return true;
            }
        }
    }
    
    // If no [lib] section, check if it's a library by file structure
    let has_lib_rs = metadata("src/lib.rs").is_ok();
    let has_main_rs = metadata("src/main.rs").is_ok();
    
    has_lib_rs && !has_main_rs
}

fn run_rust_project() {
    // Get the project's binary name from Cargo.toml
    let binary_name = get_binary_name();
    
    // Check if target directory exists
    let target_dir = "./target";
    let target_release = format!("./target/release/{}", binary_name);

    match metadata(target_dir) {
        Ok(_) => {
            // Check if release binary exists
            if !metadata(&target_release).is_ok() {
                println!("[TIP] + Nothing at `target` .");
                println!();
                println!("[1/2] + Build the project first.");

                // Run lox build
                println!("  - Task | lox build | ");
                let build_status = Command::new("cargo")
                    .arg("run")
                    .arg("--")
                    .arg("build")
                    .status()
                    .expect("Failed to execute lox build");
                println!(
                    "  - Task | lox build | {}.",
                    if build_status.success() {
                        "Done"
                    } else {
                        "Failed"
                    }
                );

                println!();
            }
        }
        Err(_) => {
            println!("[TIP] + Nothing at `target` .");
            println!();
            println!("[1/2] + Build the project first.");

            // Run lox build
            println!("  - Task | lox build | ");
            let build_status = Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("build")
                .status()
                .expect("Failed to execute lox build");
            println!(
                "  - Task | lox build | {}.",
                if build_status.success() {
                    "Done"
                } else {
                    "Failed"
                }
            );

            println!();
        }
    }

    println!("[2/2] + Run the project.");

    // Run the release binary and measure time
    println!("  - Task | {} | ", target_release);
    let start_time = Instant::now();
    let run_status = Command::new(&target_release)
        .status()
        .expect(format!("Failed to execute {}", target_release).as_str());
    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    println!(
        "  - Task | {} | {}.",
        target_release,
        if run_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[TIP] + Run the project in {:.2}s.", elapsed_seconds);
    println!("[TIP] + [Task End]");
    println!();
}

fn get_binary_name() -> String {
    if let Ok(cargo_content) = read_to_string("Cargo.toml") {
        let mut package_name = String::new();
        let mut in_package_section = false;
        
        for line in cargo_content.lines() {
            let trimmed_line = line.trim();
            
            // Check for section headers
            if trimmed_line.starts_with('[') {
                in_package_section = trimmed_line == "[package]";
                continue;
            }
            
            // Extract package name if in package section
            if in_package_section && trimmed_line.starts_with("name = ") {
                if let Some((_, rest)) = trimmed_line.split_once('"') {
                    if let Some((name, _)) = rest.split_once('"') {
                        package_name = name.replace('-', "_");
                        break;
                    }
                }
            }
        }
        
        if !package_name.is_empty() {
            return package_name;
        }
    }
    
    // Fallback to current directory name if package name not found
    if let Ok(current_dir) = std::env::current_dir() {
        if let Some(dir_name) = current_dir.file_name() {
            if let Some(dir_str) = dir_name.to_str() {
                return dir_str.replace('-', "_").to_string();
            }
        }
    }
    
    // Default fallback
    String::from("lox")
}

fn run_uv_project() {
    println!();
    println!("[1/2] + Lock the project dependencies.");

    // Run uv lock
    println!("  - Task | uv lock | ");
    let lock_status = Command::new("uv")
        .arg("lock")
        .status()
        .expect("Failed to execute uv lock");
    println!(
        "  - Task | uv lock | {}.",
        if lock_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[2/2] + Run the project.");

    // Run the project with uv run and measure time
    println!("  - Task | uv run main.py | ");
    let start_time = Instant::now();
    let run_status = Command::new("uv")
        .arg("run")
        .arg("main.py")
        .status()
        .expect("Failed to execute uv run main.py");
    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    println!(
        "  - Task | uv run main.py | {}.",
        if run_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[TIP] + Run the project in {:.2}s.", elapsed_seconds);
    println!("[TIP] + [Task End]");
    println!();
}

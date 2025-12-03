use std::fs::metadata;
use std::fs::read_to_string;
use std::process::Command;

pub fn run() {
    println!();

    // Check if it's a Rust project (has Cargo.toml)
    let is_rust_project = metadata("Cargo.toml").is_ok();

    if is_rust_project {
        // Check if it's a library project
        if is_library_project() {
            println!("[TIP] + The current project is a library(lib) project, which doesn't have binary output.");
            println!("[TIP] + [Task End]");
            println!();
            return;
        }
        // Get the project's binary name from Cargo.toml
        let binary_name = get_binary_name();
        
        // Check if target directory exists
        let target_dir = "./target";
        let target_debug = format!("./target/debug/{}", binary_name);

        match metadata(target_dir) {
            Ok(_) => {
                // Check if debug binary exists
                if !metadata(&target_debug).is_ok() {
                    println!("[TIP] + Nothing at `target` .");
                    println!();
                    println!("[1/2] + Build the project first.");

                    // Run lox dev
                    println!("  - Task | lox dev | ");
                    let dev_status = Command::new("cargo")
                        .arg("run")
                        .arg("--")
                        .arg("dev")
                        .status()
                        .expect("Failed to execute lox dev");
                    println!(
                        "  - Task | lox dev | {}.",
                        if dev_status.success() {
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

                // Run lox dev
                println!("  - Task | lox dev | ");
                let dev_status = Command::new("cargo")
                    .arg("run")
                    .arg("--")
                    .arg("dev")
                    .status()
                    .expect("Failed to execute lox dev");
                println!(
                    "  - Task | lox dev | {}.",
                    if dev_status.success() {
                        "Done"
                    } else {
                        "Failed"
                    }
                );

                println!();
            }
        }

        println!("[2/2] + Run the project.");

        // Run the debug binary
        println!("  - Task | {} | ", target_debug);
        let run_status = Command::new(&target_debug)
            .status()
            .expect(format!("Failed to execute {}", target_debug).as_str());
        println!(
            "  - Task | {} | {}.",
            target_debug,
            if run_status.success() {
                "Done"
            } else {
                "Failed"
            }
        );

        println!();
        println!("[TIP] + Run the project in 0.56s.");
        println!("[TIP] + [Task End]");
        println!();
    } else {
        println!("[TIP] + Unknown project type. No dash configuration found.");
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
    String::from("lox-rs")
}

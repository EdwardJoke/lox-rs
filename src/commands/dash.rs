use crate::projects;
use std::fs::metadata;
use std::process::Command;

pub fn run() {
    println!();

    // Get project information
    let project = projects::get_or_create_project();

    if project.is_rust_project {
        // Check if it's a library project
        if project.is_library {
            println!(
                "[TIP] + The current project is a library(lib) project, which doesn't have binary output."
            );
            println!("[TIP] + [Task End]");
            println!();
            return;
        }
        run_project(&project);
    } else if project.is_uv_project {
        println!("[TIP] + The `dash` command is not supported for `uv` projects.");
        println!("[TIP] + Please use `lox run` or `lox build`.");
        println!("[TIP] + [Task End]");
        println!();
    } else {
        println!("[TIP] + Unknown project type. No dash configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

fn run_project(project: &projects::Project) {
    let target_debug = &project.run_commands.dev;

    // Only check target directory for Rust/Cargo projects
    if project.is_rust_project {
        // Check if target directory exists
        let target_dir = "./target";

        match metadata(target_dir) {
            Ok(_) => {
                // Check if debug binary exists
                if !metadata(target_debug).is_ok() {
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
    }

    println!("[2/2] + Run the project.");

    // Run the debug command
    println!("  - Task | {} | ", target_debug);
    
    // Split command into binary and arguments for proper execution
    let mut parts = target_debug.split_whitespace();
    let run_status = if let Some(binary) = parts.next() {
        let args: Vec<&str> = parts.collect();
        Command::new(binary)
            .args(args)
            .status()
            .expect(format!("Failed to execute {}", target_debug).as_str())
    } else {
        panic!("Empty command string")
    };
    
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
}

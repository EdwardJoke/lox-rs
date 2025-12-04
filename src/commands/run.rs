use crate::projects;
use std::fs::metadata;
use std::process::Command;
use std::time::Instant;

pub fn run() {
    println!();

    // Get project information
    let project = projects::get_or_create_project();

    if project.is_rust_project || project.is_uv_project {
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
    } else {
        println!("[TIP] + Unknown project type. No run configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

fn run_project(project: &projects::Project) {
    let target_release = &project.run_commands.release;

    // Only check target directory for Rust/Cargo projects
    if project.is_rust_project {
        // Check if target directory exists
        let target_dir = "./target";

        match metadata(target_dir) {
            Ok(_) => {
                // Check if release binary exists
                if !metadata(target_release).is_ok() {
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
    } else if project.is_uv_project {
        // For UV projects, lock dependencies first
        println!("[1/2] + Lock the project dependencies.");
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
    }

    // Run the release command and measure time
    println!("  - Task | {} | ", target_release);
    let start_time = Instant::now();
    
    // Split command into binary and arguments for proper execution
    let mut parts = target_release.split_whitespace();
    let run_status = if let Some(binary) = parts.next() {
        let args: Vec<&str> = parts.collect();
        Command::new(binary)
            .args(args)
            .status()
            .expect(format!("Failed to execute {}", target_release).as_str())
    } else {
        panic!("Empty command string")
    };
    
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

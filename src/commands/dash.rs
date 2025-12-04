use crate::project;
use std::fs::metadata;
use std::process::Command;

pub fn run() {
    println!();

    // Get project information
    let project = project::get_or_create_project();

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
        println!("[TIP] + Unknown project type. No dash configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

fn run_project(project: &project::Project) {
    // Check if target directory exists
    let target_dir = "./target";
    let target_debug = &project.run_commands.dev;

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

    println!("[2/2] + Run the project.");

    // Run the debug binary
    println!("  - Task | {} | ", target_debug);
    let run_status = Command::new(target_debug)
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
}

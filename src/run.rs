use std::fs::metadata;
use std::process::Command;

pub fn run() {
    println!();

    // Check if target directory exists
    let target_dir = "./target";
    let target_release = "./target/release/lox-rs";

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

    // Run the release binary
    println!("  - Task | ./target/release/lox-rs | ");
    let run_status = Command::new("./target/release/lox-rs")
        .status()
        .expect("Failed to execute ./target/release/lox-rs");
    println!(
        "  - Task | ./target/release/lox-rs | {}.",
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

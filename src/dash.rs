use std::fs::metadata;
use std::process::Command;

pub fn run() {
    println!();

    // Check if target directory exists
    let target_dir = "./target";
    let target_debug = "./target/debug/lox-rs";

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
    println!("  - Task | ./target/debug/lox-rs | ");
    let run_status = Command::new("./target/debug/lox-rs")
        .status()
        .expect("Failed to execute ./target/debug/lox-rs");
    println!(
        "  - Task | ./target/debug/lox-rs | {}.",
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

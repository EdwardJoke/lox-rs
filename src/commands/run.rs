use std::fs::metadata;
use std::process::Command;
use std::time::Instant;

pub fn run() {
    println!();

    // Check if it's a Rust project (has Cargo.toml)
    let is_rust_project = metadata("Cargo.toml").is_ok();

    // Check if it's a Python project (has pyproject.toml)
    let is_uv_project = metadata("pyproject.toml").is_ok();

    if is_rust_project {
        run_rust_project();
    } else if is_uv_project {
        run_uv_project();
    } else {
        println!("[TIP] + Unknown project type. No run configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

fn run_rust_project() {
    // Check if target directory exists
    let target_dir = "./target";
    let target_release = "./target/release/lox";

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

    // Run the release binary and measure time
    println!("  - Task | ./target/release/lox | ");
    let start_time = Instant::now();
    let run_status = Command::new("./target/release/lox")
        .status()
        .expect("Failed to execute ./target/release/lox");
    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    println!(
        "  - Task | ./target/release/lox | {}.",
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

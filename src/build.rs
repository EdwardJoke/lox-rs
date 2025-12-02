use std::process::Command;

pub fn run() {
    println!();
    println!("[TIP] + Build for Release.");
    println!();
    println!("[1/3] + Download dependencies");

    // Run cargo update
    println!("  - Task | cargo update | ");
    let update_status = Command::new("cargo")
        .arg("update")
        .status()
        .expect("Failed to execute cargo update");
    println!(
        "  - Task | cargo update | {}.",
        if update_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    // Run cargo fmt
    println!("  - Task | cargo fmt    | ");
    let fmt_status = Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("Failed to execute cargo fmt");
    println!(
        "  - Task | cargo fmt    | {}.",
        if fmt_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[2/3] + Check the project");

    // Run cargo check
    println!("  - Task | cargo check  | ");
    let check_status = Command::new("cargo")
        .arg("check")
        .status()
        .expect("Failed to execute cargo check");
    println!(
        "  - Task | cargo check  | {}.",
        if check_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[3/3] + Build the project");

    // Run cargo build --release
    println!("  - Task | cargo build --release | ");
    let build_status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()
        .expect("Failed to execute cargo build --release");
    println!(
        "  - Task | cargo build --release | {}.",
        if build_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[TIP] + Build at + `./target/release/lox-rs` .");
    println!("[TIP] + [Task End]");
    println!();
}

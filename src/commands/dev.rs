use crate::project;
use std::process::Command;

pub fn run() {
    println!();

    // Get project information
    let project = project::get_or_create_project();

    if project.is_rust_project {
        build_dev_rust_project(&project);
    } else {
        println!("[TIP] + Unknown project type. No dev configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

fn build_dev_rust_project(project: &project::Project) {
    println!("[TIP] + Build for Dev.");
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

    // Run cargo build
    println!("  - Task | {} | ", project.build_commands.dev);
    let build_status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to execute cargo build");
    println!(
        "  - Task | {} | {}.",
        project.build_commands.dev,
        if build_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[TIP] + Build at + `target` .");
    println!("[TIP] + [Task End]");
    println!();
}

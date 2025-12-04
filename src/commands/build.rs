use crate::project;
use std::process::Command;

pub fn run() {
    println!();

    // Get project information
    let project = project::get_or_create_project();

    if project.is_rust_project || project.is_uv_project {
        build_project(&project);
    } else {
        println!("[TIP] + Unknown project type. No build configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

fn build_project(project: &project::Project) {
    if project.is_rust_project {
        build_rust_project(project);
    } else if project.is_uv_project {
        build_uv_project(project);
    }
}

fn build_rust_project(project: &project::Project) {
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
    println!("  - Task | {} | ", project.build_commands.release);
    let build_status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()
        .expect("Failed to execute cargo build --release");
    println!(
        "  - Task | {} | {}.",
        project.build_commands.release,
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

fn build_uv_project(project: &project::Project) {
    println!("[TIP] + Build the project.");
    println!();
    println!("[1/3] + Lock the project dependencies");

    // Run uv lock
    println!("  - Task | uv lock         | ");
    let lock_status = Command::new("uv")
        .arg("lock")
        .status()
        .expect("Failed to execute uv lock");
    println!(
        "  - Task | uv lock         | {}.",
        if lock_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[2/3] + Check and Format the project");

    // Run uvx ruff check
    println!("  - Task | uvx ruff check  | ");
    let check_status = Command::new("uvx")
        .arg("ruff")
        .arg("check")
        .status()
        .expect("Failed to execute uvx ruff check");
    println!(
        "  - Task | uvx ruff check  | {}.",
        if check_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    // Run uvx ruff format
    println!("  - Task | uvx ruff format | ");
    let format_status = Command::new("uvx")
        .arg("ruff")
        .arg("format")
        .status()
        .expect("Failed to execute uvx ruff format");
    println!(
        "  - Task | uvx ruff format | {}.",
        if format_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[3/3] + Build the project");

    // Run uv build
    println!("  - Task | {} | ", project.build_commands.release);
    let build_status = Command::new("uv")
        .arg("build")
        .status()
        .expect("Failed to execute uv build");
    println!(
        "  - Task | {} | {}.",
        project.build_commands.release,
        if build_status.success() {
            "Done"
        } else {
            "Failed"
        }
    );

    println!();
    println!("[TIP] + Build at + `dist` .");
    println!("[TIP] + [Task End]");
    println!();
}

use std::env;
use std::fs::{read_to_string, write};
use std::process::Command;

pub fn run() {
    println!();

    // Check if this is the first run by looking for the config file
    let is_first_run = !std::fs::metadata("lox.toml").is_ok();

    if is_first_run {
        println!("[TIP] + Never run the doctor command in the project before.");
    }

    println!();
    println!("[1/2] + Project informations");

    // Parse Cargo.toml for project information
    let mut project_type = String::from("unknown");
    let mut project_name = String::from("unknown");
    let mut project_version = String::from("unknown");

    // Extract project information from Cargo.toml
    if let Ok(cargo_content) = read_to_string("Cargo.toml") {
        for line in cargo_content.lines() {
            if line.starts_with("name = ") {
                if let Some((_, rest)) = line.split_once('"') {
                    if let Some((name, _)) = rest.split_once('"') {
                        project_name = name.to_string();
                    }
                }
            } else if line.starts_with("version = ") {
                if let Some((_, rest)) = line.split_once('"') {
                    if let Some((version, _)) = rest.split_once('"') {
                        project_version = version.to_string();
                    }
                }
            } else if line.starts_with("[lib]") {
                project_type = String::from("library(lib)");
            } else if line.starts_with("[[bin]]") {
                project_type = String::from("app(bin)");
            }
        }
    }

    // If no explicit type found, default to binary if there's a main.rs
    if project_type == "unknown" {
        project_type = if std::fs::metadata("src/main.rs").is_ok() {
            String::from("app(bin)")
        } else if std::fs::metadata("src/lib.rs").is_ok() {
            String::from("library(lib)")
        } else {
            String::from("unknown")
        };
    }

    // Check if it's a Rust project (has Cargo.toml)
    let is_rust_project = std::fs::metadata("Cargo.toml").is_ok();
    
    // Display project type with conditional Rust suffix
    if is_rust_project {
        println!("  - Project type:           {} (rust)", project_type);
    } else {
        println!("  - Project type:           {}", project_type);
    }
    println!("  - Project name:           {}", project_name);
    println!("  - Project version:        {}", project_version);

    // Only display Rust-specific commands if it's a Rust project
    if is_rust_project {
        println!("  - Project build(dev):     cargo build");
        println!("  - Project build(release): cargo build --release");
        println!("  - Project fmt:            cargo fmt");
        println!("  - Project lint:           cargo check");
        println!("  - Project dependency:     cargo update");
    } else {
        println!("  - Project build(dev):     unknown");
        println!("  - Project build(release): unknown");
        println!("  - Project fmt:            unknown");
        println!("  - Project lint:           unknown");
        println!("  - Project dependency:     unknown");
    }
    println!();
    println!("[2/2] + Environment informations");

    // Get OS information
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let formatted_os = format_os_name(os);

    println!("  - Operating system:      {}", formatted_os);
    println!("  - CPU architecture:      {}", arch);

    // Get RustC and Cargo versions (only used if it's a Rust project)
    let (rustc_version, cargo_version) = if is_rust_project {
        // Get RustC version
        let rustc_output = Command::new("rustc")
            .arg("--version")
            .output()
            .expect("Failed to execute rustc command");
        let rustc_version_str = String::from_utf8_lossy(&rustc_output.stdout);
        let rustc_version = rustc_version_str
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap_or("unknown")
            .to_string();

        // Get Cargo version
        let cargo_output = Command::new("cargo")
            .arg("--version")
            .output()
            .expect("Failed to execute cargo command");
        let cargo_version_str = String::from_utf8_lossy(&cargo_output.stdout);
        let cargo_version = cargo_version_str
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap_or("unknown")
            .to_string();

        println!("  - RustC version:         {}", rustc_version);
        println!("  - Cargo version:         {}", cargo_version);

        (rustc_version, cargo_version)
    } else {
        ("unknown".to_string(), "unknown".to_string())
    };
    println!();

    // Create and write to lox.toml only on first run
    if is_first_run {
        // Create TOML content for project configuration
        let mut toml_content = format!(
            "\n[project]\ntype = \"{}\"\nname = \"{}\"\nversion = \"{}\"\n",
            project_type, project_name, project_version
        );

        // Add Rust-specific build commands if it's a Rust project
        if is_rust_project {
            toml_content.push_str(
                "\n[project.build]\ndev = \"cargo build\"\nrelease = \"cargo build --release\"\n",
            );
            toml_content.push_str("\n[project.commands]\nfmt = \"cargo fmt\"\nlint = \"cargo check\"\ndependency = \"cargo update\"\n");
            toml_content.push_str(format!("\n[environment]\nos = \"{}\"\narch = \"{}\"\nrustc_version = \"{}\"\ncargo_version = \"{}\"\n",
                formatted_os, arch, rustc_version, cargo_version).as_str());
        } else {
            toml_content.push_str("\n[project.build]\ndev = \"unknown\"\nrelease = \"unknown\"\n");
            toml_content.push_str("\n[project.commands]\nfmt = \"unknown\"\nlint = \"unknown\"\ndependency = \"unknown\"\n");
            toml_content.push_str(
                format!(
                    "\n[environment]\nos = \"{}\"\narch = \"{}\"\n",
                    formatted_os, arch
                )
                .as_str(),
            );
        }

        // Write the configuration to lox.toml
        if let Err(e) = write("lox.toml", toml_content) {
            eprintln!(
                "Warning: Failed to write project configuration to lox.toml: {}",
                e
            );
        } else {
            println!("[TIP] + Project configuration saved to `lox.toml`.");
        }
    }

    println!("[TIP] + Everything is Up-to-date.");
    println!("[TIP] + [Task End]");
    println!();
}

fn format_os_name(os: &str) -> String {
    match os {
        "macos" => "macOS".to_string(),
        "linux" => "Linux".to_string(),
        "windows" => "Windows".to_string(),
        _ => os.to_string(),
    }
}

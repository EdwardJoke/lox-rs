use crate::projects;
use std::env;
use std::fs::write;
use std::time::Instant;
use tokio::process::Command;

pub async fn run(quiet: bool) {
    // Start timer for all tasks
    let start_time = Instant::now();

    // Check if this is the first run by looking for the config file
    let is_first_run = !std::fs::metadata("lox.toml").is_ok();

    // Get project information from the shared module
    let project = projects::detect_project_info();

    // Get Rust-specific information for Rust projects
    let (rustc_version, cargo_version) = if project.is_rust_project {
        // Get RustC version
        let rustc_output = Command::new("rustc")
            .arg("--version")
            .output()
            .await
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
            .await
            .expect("Failed to execute cargo command");
        let cargo_version_str = String::from_utf8_lossy(&cargo_output.stdout);
        let cargo_version = cargo_version_str
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap_or("unknown")
            .to_string();

        (rustc_version, cargo_version)
    } else {
        ("unknown".to_string(), "unknown".to_string())
    };

    // Get Python-specific information for Python projects
    let mut uv_version = "unknown".to_string();
    if project.is_uv_project {
        // Get uv version
        if let Ok(uv_output) = Command::new("uv").arg("--version").output().await {
            let uv_version_str = String::from_utf8_lossy(&uv_output.stdout);
            uv_version = uv_version_str
                .trim()
                .split_whitespace()
                .nth(1)
                .unwrap_or("unknown")
                .to_string();
        }
    }

    // Create and write to lox.toml only on first run
    if is_first_run {
        // Create TOML content for project configuration
        let mut toml_content = format!(
            "[project]\ntype = \"{}\"\nname = \"{}\"\nversion = \"{}\"\n",
            project.project_type, project.name, project.version
        );

        // Add Rust-specific build commands if it's a Rust project
        if project.is_rust_project {
            toml_content.push_str(
                format!(
                    "\n[project.build]\ndev = \"{}\"\nrelease = \"{}\"\n",
                    project.build_commands.dev, project.build_commands.release
                )
                .as_str(),
            );
            toml_content.push_str(
                format!(
                    "\n[project.run]\ndev = \"{}\"\nrelease = \"{}\"\n",
                    project.run_commands.dev, project.run_commands.release
                )
                .as_str(),
            );
            toml_content.push_str("\n[project.commands]\nfmt = \"cargo fmt\"\nlint = \"cargo check\"\ndependency = \"cargo update\"\n");
            toml_content.push_str(format!("\n[environment]\nos = \"{}\"\narch = \"{}\"\nrustc_version = \"{}\"\ncargo_version = \"{}\"\n",
                projects::format_os_name(env::consts::OS), env::consts::ARCH, rustc_version, cargo_version).as_str());
        } else if project.is_uv_project {
            // Add Python-specific commands for Python projects
            toml_content.push_str(
                format!(
                    "\n[project.build]\ndev = \"{}\"\nrelease = \"{}\"\n",
                    project.build_commands.dev, project.build_commands.release
                )
                .as_str(),
            );
            toml_content.push_str(
                format!(
                    "\n[project.run]\ndev = \"{}\"\nrelease = \"{}\"\n",
                    project.run_commands.dev, project.run_commands.release
                )
                .as_str(),
            );
            toml_content.push_str("\n[project.commands]\nfmt = \"uvx ruff format\"\nlint = \"uvx ruff check\"\ndependency = \"uv update\"\n");
            toml_content.push_str(
                format!(
                    "\n[environment]\nos = \"{}\"\narch = \"{}\"\nuv_version = \"{}\"\n",
                    projects::format_os_name(env::consts::OS), env::consts::ARCH, uv_version
                )
                .as_str(),
            );
        } else {
            // Default for other project types
            toml_content.push_str(
                format!(
                    "\n[project.build]\ndev = \"{}\"\nrelease = \"{}\"\n",
                    project.build_commands.dev, project.build_commands.release
                )
                .as_str(),
            );
            toml_content.push_str("\n[project.commands]\nfmt = \"unknown\"\nlint = \"unknown\"\ndependency = \"unknown\"\n");
            toml_content.push_str(
                format!(
                    "\n[environment]\nos = \"{}\"\narch = \"{}\"\n",
                    projects::format_os_name(env::consts::OS), env::consts::ARCH
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
        } else if !quiet {
            println!("[TIP] + Project configuration saved to `lox.toml`.");
        }
    }

    // Only print detailed output if not in quiet mode
    if !quiet {
        println!();

        if is_first_run {
            println!("[TIP] + Never run the doctor command in the project before.");
        }

        println!();
        println!("[1/2] + Project informations");

        // Display project type with conditional suffix
        if project.is_rust_project {
            println!(
                "  - Project type:           {} (rust)",
                project.project_type
            );
        } else if project.is_uv_project {
            println!(
                "  - Project type:           {} (python)",
                project.project_type
            );
        } else {
            println!("  - Project type:           {}", project.project_type);
        }
        println!("  - Project name:           {}", project.name);
        println!("  - Project version:        {}", project.version);

        // Display project virtual env for Python projects
        if project.is_uv_project {
            println!("  - Project virtual env:    unknown");
        }

        // Display project commands based on project type
        if project.is_rust_project {
            println!("  - Project build(dev):     {}", project.build_commands.dev);
            println!(
                "  - Project build(release): {}",
                project.build_commands.release
            );
            println!("  - Project fmt:            cargo fmt");
            println!("  - Project lint:           cargo check");
            println!("  - Project dependency:     cargo update");
        } else if project.is_uv_project {
            println!("  - Project build:          {}", project.build_commands.dev);
            println!("  - Project fmt:            uvx ruff format");
            println!("  - Project lint:           uvx ruff check");
            println!("  - Project dependency:     uv update");
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
        let formatted_os = projects::format_os_name(os);

        println!("  - Operating system:      {}", formatted_os);
        println!("  - CPU architecture:      {}", arch);

        if project.is_rust_project {
            println!("  - RustC version:         {}", rustc_version);
            println!("  - Cargo version:         {}", cargo_version);
        }

        if project.is_uv_project {
            println!("  - uv version:            {}", uv_version);
        }
        println!();

        println!("[TIP] + Everything is Up-to-date.");

        // Calculate and display total elapsed time
        let elapsed = start_time.elapsed();
        let elapsed_seconds = elapsed.as_secs_f64();
        println!("[TIP] + Done the tasks in {:.2}s.", elapsed_seconds);

        println!("[TIP] + [Task End]");
        println!();
    }
}

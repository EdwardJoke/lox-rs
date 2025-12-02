use std::env;
use std::fs::{read_to_string, File};
use std::process::Command;

pub fn run() {
    println!();
    
    // Check if this is the first run by looking for a marker file
    let is_first_run = !std::fs::metadata("loxproject.toml").is_ok();
    
    if is_first_run {
        println!("[TIP] + Never run the doctor command in the project before.");
        // Create the marker file
        if let Err(e) = File::create("loxproject.toml") {
            eprintln!("Warning: Failed to create first run marker: {}", e);
        }
    }
    
    println!();
    println!("[1/2] + Project informations");
    
    // Parse Cargo.toml for project information
    if let Ok(cargo_toml) = read_to_string("Cargo.toml") {
        let mut project_name = "unknown";
        let mut project_version = "unknown";
        let mut project_type = "unknown";
        
        // Extract project information from Cargo.toml
        for line in cargo_toml.lines() {
            if line.starts_with("name = ") {
                project_name = line.split_once('"').and_then(|(_, rest)| rest.split_once('"')).map(|(name, _)| name).unwrap_or("unknown");
            } else if line.starts_with("version = ") {
                project_version = line.split_once('"').and_then(|(_, rest)| rest.split_once('"')).map(|(version, _)| version).unwrap_or("unknown");
            } else if line.starts_with("[lib]") {
                project_type = "library(lib)";
            } else if line.starts_with("[[bin]]") {
                project_type = "application(bin)";
            }
        }
        
        // If no explicit type found, default to binary if there's a main.rs
        if project_type == "unknown" {
            project_type = if std::fs::metadata("src/main.rs").is_ok() {
                "application(bin)"
            } else if std::fs::metadata("src/lib.rs").is_ok() {
                "library(lib)"
            } else {
                "unknown"
            };
        }
        
        println!("  - Project type:           {}", project_type);
        println!("  - Project name:           {}", project_name);
        println!("  - Project version:        {}", project_version);
    } else {
        println!("  - Project type:           unknown");
        println!("  - Project name:           unknown");
        println!("  - Project version:        unknown");
    }
    
    println!("  - Project build(dev):     cargo build");
    println!("  - Project build(release): cargo build --release");
    println!();
    println!("[2/2] + Environment informations");

    // Get OS information
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

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
        .unwrap_or("unknown");

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
        .unwrap_or("unknown");

    println!("  - Operating system:      {}", format_os_name(os));
    println!("  - CPU architecture:      {}", arch);
    println!("  - RustC version:         {}", rustc_version);
    println!("  - Cargo version:         {}", cargo_version);
    println!();
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

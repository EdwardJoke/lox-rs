use std::future::Future;
use std::pin::Pin;
use tokio::process::Command;
use std::io::{stdin, stdout, Write};

// Define a type alias for our async task function
pub type TaskFn = dyn Fn() -> Pin<Box<dyn Future<Output = bool>>> + Send + Sync;

// Define a Task struct that encapsulates task logic
struct Task {
    // Unique task ID
    id: String,
    // Display name of the task
    name: String,
    // Async function to execute the task
    execute: Box<TaskFn>,
}

impl Task {
    // Create a new task with the given ID, name, and async function
    fn new<F, Fut>(id: &str, name: &str, execute: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = bool> + 'static,
    {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            execute: Box::new(move || Box::pin(execute())),
        }
    }

    // Get the unique task ID
    fn id(&self) -> &str {
        &self.id
    }

    // Get the display name of the task
    fn name(&self) -> &str {
        &self.name
    }

    // Execute the task and return its status
    async fn execute(&self) -> bool {
        (self.execute)().await
    }

    // Get the message to display after task completion
    fn get_result_message(&self, success: bool) -> String {
        format!(
            "  - Task | {} | {}.",
            self.name(),
            if success { "Done" } else { "Failed" }
        )
    }
}

// Helper functions for UV installation

// Check if UV is installed
async fn is_uv_installed() -> bool {
    Command::new("uv")
        .arg("--version")
        .status()
        .await
        .is_ok()
}

// Install UV based on the operating system
async fn install_uv() -> bool {
    println!("[TIP] + Seems like you didn't install `uv` yet.");
    println!("[TIP] + Do you want to install `uv` now? (Y/n) >> ");
    
    // Flush stdout to ensure the prompt is displayed immediately
    stdout().flush().expect("Failed to flush stdout");
    
    // Read user input
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    
    let input = input.trim().to_lowercase();
    if input != "y" && input != "yes" && !input.is_empty() {
        println!("[TIP] + Installation canceled by user.");
        return false;
    }
    
    // Determine OS and install UV
    let os = std::env::consts::OS;
    match os {
        "macos" | "linux" => {
            println!("  - Task | curl -LsSf https://astral.sh/uv/install.sh | sh | ");
            let status = Command::new("sh")
                .arg("-c")
                .arg("curl -LsSf https://astral.sh/uv/install.sh | sh")
                .status()
                .await
                .expect("Failed to execute UV installation script");
            println!("  - Task | curl -LsSf https://astral.sh/uv/install.sh | sh | Done.");
            status.success()
        },
        "windows" => {
            println!("  - Task | powershell -ExecutionPolicy ByPass -c \"irm https://astral.sh/uv/install.ps1 | iex\" | ");
            let status = Command::new("powershell")
                .arg("-ExecutionPolicy")
                .arg("ByPass")
                .arg("-c")
                .arg("irm https://astral.sh/uv/install.ps1 | iex")
                .status()
                .await
                .expect("Failed to execute UV installation script");
            println!("  - Task | powershell -ExecutionPolicy ByPass -c \"irm https://astral.sh/uv/install.ps1 | iex\" | Done.");
            status.success()
        },
        _ => {
            eprintln!("[ERROR] + Unsupported operating system for UV installation.");
            false
        }
    }
}

// Execute UV command with installation check
async fn execute_uv_command(args: &[&str]) -> bool {
    // Check if UV is installed
    if !is_uv_installed().await {
        // Install UV if not installed
        if !install_uv().await {
            return false;
        }
        println!("[TIP] + `uv` already installed, please restart the terminal.");
        return false;
    }
    
    // Execute the UV command
    Command::new("uv")
        .args(args)
        .status()
        .await
        .expect("Failed to execute UV command")
        .success()
}

// Concrete task factories

// Create a UV lock task
fn create_uv_lock_task() -> Task {
    Task::new("uv_lock", "uv lock", || async {
        execute_uv_command(&["lock"]).await
    })
}

// Create a UV run task
fn create_uv_run_task() -> Task {
    Task::new("uv_run", "uv run main.py", || async {
        execute_uv_command(&["run", "main.py"]).await
    })
}

// Create a UV build task
fn create_uv_build_task() -> Task {
    Task::new("uv_build", "uv build", || async {
        execute_uv_command(&["build"]).await
    })
}

// Create a UV ruff check task
fn create_uv_ruff_check_task() -> Task {
    Task::new("uv_ruff_check", "uvx ruff check", || async {
        // Check if UV is installed first, since uvx is part of UV
        if !is_uv_installed().await {
            if !install_uv().await {
                return false;
            }
            println!("[TIP] + `uv` already installed, please restart the terminal.");
            return false;
        }
        Command::new("uvx")
            .arg("ruff")
            .arg("check")
            .status()
            .await
            .expect("Failed to execute uvx ruff check")
            .success()
    })
}

// Create a UV ruff format task
fn create_uv_ruff_format_task() -> Task {
    Task::new("uv_ruff_format", "uvx ruff format", || async {
        // Check if UV is installed first, since uvx is part of UV
        if !is_uv_installed().await {
            if !install_uv().await {
                return false;
            }
            println!("[TIP] + `uv` already installed, please restart the terminal.");
            return false;
        }
        Command::new("uvx")
            .arg("ruff")
            .arg("format")
            .status()
            .await
            .expect("Failed to execute uvx ruff format")
            .success()
    })
}

// Cargo project tasks

// Create a cargo update task
fn create_cargo_update_task() -> Task {
    Task::new("cargo_update", "cargo update", || async {
        Command::new("cargo")
            .arg("update")
            .status()
            .await
            .expect("Failed to execute cargo update")
            .success()
    })
}

// Create a cargo fmt task
fn create_cargo_fmt_task() -> Task {
    Task::new("cargo_fmt", "cargo fmt", || async {
        Command::new("cargo")
            .arg("fmt")
            .status()
            .await
            .expect("Failed to execute cargo fmt")
            .success()
    })
}

// Create a cargo check task
fn create_cargo_check_task() -> Task {
    Task::new("cargo_check", "cargo check", || async {
        Command::new("cargo")
            .arg("check")
            .status()
            .await
            .expect("Failed to execute cargo check")
            .success()
    })
}

// Create a cargo build task
fn create_cargo_build_task() -> Task {
    Task::new("cargo_build", "cargo build", || async {
        Command::new("cargo")
            .arg("build")
            .status()
            .await
            .expect("Failed to execute cargo build")
            .success()
    })
}

// Create a cargo build release task
fn create_cargo_build_release_task() -> Task {
    Task::new("cargo_build_release", "cargo build --release", || async {
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .status()
            .await
            .expect("Failed to execute cargo build --release")
            .success()
    })
}

// Task registry to store and retrieve tasks by ID
struct TaskRegistry {
    tasks: Vec<Box<Task>>,
}

impl TaskRegistry {
    fn new() -> Self {
        let tasks: Vec<Box<Task>> = vec![
            Box::new(create_uv_lock_task()),
            Box::new(create_uv_run_task()),
            Box::new(create_uv_build_task()),
            Box::new(create_uv_ruff_check_task()),
            Box::new(create_uv_ruff_format_task()),
            Box::new(create_cargo_update_task()),
            Box::new(create_cargo_fmt_task()),
            Box::new(create_cargo_check_task()),
            Box::new(create_cargo_build_task()),
            Box::new(create_cargo_build_release_task()),
        ];

        Self { tasks }
    }

    fn get_task_by_id(&self, id: &str) -> Option<&Box<Task>> {
        self.tasks.iter().find(|task| task.id() == id)
    }

    async fn execute_task_by_id(&self, id: &str) -> bool {
        if let Some(task) = self.get_task_by_id(id) {
            println!("  - Task | {} | ", task.name());
            let success = task.execute().await;
            println!("{}", task.get_result_message(success));
            success
        } else {
            eprintln!("Error: Task with ID '{}' not found", id);
            false
        }
    }
}

// Public API for the task system
pub async fn execute_task_by_id(task_id: &str) -> bool {
    let registry = TaskRegistry::new();
    registry.execute_task_by_id(task_id).await
}

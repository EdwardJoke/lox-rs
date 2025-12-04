use std::process::Command;

// Define a Task struct that encapsulates task logic
trait Task {
    // Get the unique task ID
    fn id(&self) -> &str;
    
    // Get the display name of the task
    fn name(&self) -> &str;
    
    // Execute the task and return its status
    fn execute(&self) -> bool;
    
    // Get the message to display after task completion
    fn get_result_message(&self, success: bool) -> String {
        format!("  - Task | {} | {}.", self.name(), if success { "Done" } else { "Failed" })
    }
}

// Concrete task implementations

// UV project tasks

struct UvLockTask;
impl Task for UvLockTask {
    fn id(&self) -> &str { "uv_lock" }
    fn name(&self) -> &str { "uv lock" }
    fn execute(&self) -> bool {
        Command::new("uv")
            .arg("lock")
            .status()
            .expect("Failed to execute uv lock")
            .success()
    }
}

struct UvRunTask;
impl Task for UvRunTask {
    fn id(&self) -> &str { "uv_run" }
    fn name(&self) -> &str { "uv run main.py" }
    fn execute(&self) -> bool {
        Command::new("uv")
            .arg("run")
            .arg("main.py")
            .status()
            .expect("Failed to execute uv run main.py")
            .success()
    }
}

struct UvBuildTask;
impl Task for UvBuildTask {
    fn id(&self) -> &str { "uv_build" }
    fn name(&self) -> &str { "uv build" }
    fn execute(&self) -> bool {
        Command::new("uv")
            .arg("build")
            .status()
            .expect("Failed to execute uv build")
            .success()
    }
}

struct UvRuffCheckTask;
impl Task for UvRuffCheckTask {
    fn id(&self) -> &str { "uv_ruff_check" }
    fn name(&self) -> &str { "uvx ruff check" }
    fn execute(&self) -> bool {
        Command::new("uvx")
            .arg("ruff")
            .arg("check")
            .status()
            .expect("Failed to execute uvx ruff check")
            .success()
    }
}

struct UvRuffFormatTask;
impl Task for UvRuffFormatTask {
    fn id(&self) -> &str { "uv_ruff_format" }
    fn name(&self) -> &str { "uvx ruff format" }
    fn execute(&self) -> bool {
        Command::new("uvx")
            .arg("ruff")
            .arg("format")
            .status()
            .expect("Failed to execute uvx ruff format")
            .success()
    }
}

// Cargo project tasks

struct CargoUpdateTask;
impl Task for CargoUpdateTask {
    fn id(&self) -> &str { "cargo_update" }
    fn name(&self) -> &str { "cargo update" }
    fn execute(&self) -> bool {
        Command::new("cargo")
            .arg("update")
            .status()
            .expect("Failed to execute cargo update")
            .success()
    }
}

struct CargoFmtTask;
impl Task for CargoFmtTask {
    fn id(&self) -> &str { "cargo_fmt" }
    fn name(&self) -> &str { "cargo fmt" }
    fn execute(&self) -> bool {
        Command::new("cargo")
            .arg("fmt")
            .status()
            .expect("Failed to execute cargo fmt")
            .success()
    }
}

struct CargoCheckTask;
impl Task for CargoCheckTask {
    fn id(&self) -> &str { "cargo_check" }
    fn name(&self) -> &str { "cargo check" }
    fn execute(&self) -> bool {
        Command::new("cargo")
            .arg("check")
            .status()
            .expect("Failed to execute cargo check")
            .success()
    }
}

struct CargoBuildTask;
impl Task for CargoBuildTask {
    fn id(&self) -> &str { "cargo_build" }
    fn name(&self) -> &str { "cargo build" }
    fn execute(&self) -> bool {
        Command::new("cargo")
            .arg("build")
            .status()
            .expect("Failed to execute cargo build")
            .success()
    }
}

struct CargoBuildReleaseTask;
impl Task for CargoBuildReleaseTask {
    fn id(&self) -> &str { "cargo_build_release" }
    fn name(&self) -> &str { "cargo build --release" }
    fn execute(&self) -> bool {
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .status()
            .expect("Failed to execute cargo build --release")
            .success()
    }
}

// Task registry to store and retrieve tasks by ID
struct TaskRegistry {
    tasks: Vec<Box<dyn Task>>,
}

impl TaskRegistry {
    fn new() -> Self {
        let tasks: Vec<Box<dyn Task>> = vec![
            Box::new(UvLockTask),
            Box::new(UvRunTask),
            Box::new(UvBuildTask),
            Box::new(UvRuffCheckTask),
            Box::new(UvRuffFormatTask),
            Box::new(CargoUpdateTask),
            Box::new(CargoFmtTask),
            Box::new(CargoCheckTask),
            Box::new(CargoBuildTask),
            Box::new(CargoBuildReleaseTask),
        ];
        
        Self { tasks }
    }
    
    fn get_task_by_id(&self, id: &str) -> Option<&Box<dyn Task>> {
        self.tasks.iter().find(|task| task.id() == id)
    }
    
    fn execute_task_by_id(&self, id: &str) -> bool {
        if let Some(task) = self.get_task_by_id(id) {
            println!("  - Task | {} | ", task.name());
            let success = task.execute();
            println!("{}", task.get_result_message(success));
            success
        } else {
            eprintln!("Error: Task with ID '{}' not found", id);
            false
        }
    }
}

// Public API for the task system
pub fn execute_task_by_id(task_id: &str) -> bool {
    let registry = TaskRegistry::new();
    registry.execute_task_by_id(task_id)
}

// Re-export the task system API for external use

// Task trait and basic types
pub use self::task::Task;

// Public API functions
pub use self::task::{execute_tasks, execute_task_by_id, get_task_ids_for_command};

// Re-export task IDs for easy access
pub const UV_LOCK: &str = "uv_lock";
pub const UV_RUN: &str = "uv_run";
pub const UV_BUILD: &str = "uv_build";
pub const UV_RUFF_CHECK: &str = "uv_ruff_check";
pub const UV_RUFF_FORMAT: &str = "uv_ruff_format";
pub const CARGO_UPDATE: &str = "cargo_update";
pub const CARGO_FMT: &str = "cargo_fmt";
pub const CARGO_CHECK: &str = "cargo_check";
pub const CARGO_BUILD: &str = "cargo_build";
pub const CARGO_BUILD_RELEASE: &str = "cargo_build_release";

// Private module containing the implementation
mod task;
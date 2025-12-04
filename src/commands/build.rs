use crate::projects;
use crate::tasks;
use std::time::Instant;

pub async fn run() {
    println!();

    // Get project information
    let project = projects::get_or_create_project();

    if project.is_rust_project || project.is_uv_project {
        build_project(&project).await;
    } else {
        println!("[TIP] + Unknown project type. No build configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

async fn build_project(project: &projects::Project) {
    if project.is_rust_project {
        build_rust_project(project).await;
    } else if project.is_uv_project {
        build_uv_project(project).await;
    }
}

async fn build_rust_project(_project: &projects::Project) {
    println!("[TIP] + Build for Release.");
    println!();

    // Start timer for all tasks
    let start_time = Instant::now();

    println!("[1/3] + Download dependencies");

    // Execute tasks using the new task system
    tasks::execute_task_by_id(tasks::CARGO_UPDATE).await;
    tasks::execute_task_by_id(tasks::CARGO_FMT).await;

    println!();
    println!("[2/3] + Check the project");
    tasks::execute_task_by_id(tasks::CARGO_CHECK).await;

    println!();
    println!("[3/3] + Build the project");
    tasks::execute_task_by_id(tasks::CARGO_BUILD_RELEASE).await;

    println!();
    println!("[TIP] + Build at + `target` .");

    // Calculate and display total elapsed time
    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    println!("[TIP] + Done the tasks in {:.2}s.", elapsed_seconds);

    println!("[TIP] + [Task End]");
    println!();
}

async fn build_uv_project(_project: &projects::Project) {
    println!("[TIP] + Build the project.");
    println!();

    // Start timer for all tasks
    let start_time = Instant::now();

    println!("[1/3] + Lock the project dependencies");

    // Execute tasks using the new task system
    tasks::execute_task_by_id(tasks::UV_LOCK).await;

    println!();
    println!("[2/3] + Check and Format the project");
    tasks::execute_task_by_id(tasks::UV_RUFF_CHECK).await;
    tasks::execute_task_by_id(tasks::UV_RUFF_FORMAT).await;

    println!();
    println!("[3/3] + Build the project");
    tasks::execute_task_by_id(tasks::UV_BUILD).await;

    println!();
    println!("[TIP] + Build at + `dist` .");

    // Calculate and display total elapsed time
    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    println!("[TIP] + Done the tasks in {:.2}s.", elapsed_seconds);

    println!("[TIP] + [Task End]");
    println!();
}

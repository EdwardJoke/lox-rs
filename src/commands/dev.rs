use crate::projects;
use crate::tasks;
use std::time::Instant;

pub async fn run() {
    println!();

    // Get project information
    let project = projects::get_or_create_project();

    if project.is_rust_project {
        build_dev_rust_project(&project).await;
    } else if project.is_uv_project {
        println!("[TIP] + The `dev` command is not supported for `uv` projects.");
        println!("[TIP] + Please use `lox run` or `lox build`.");
        println!("[TIP] + [Task End]");
        println!();
    } else {
        println!("[TIP] + Unknown project type. No dev configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

async fn build_dev_rust_project(_project: &projects::Project) {
    println!("[TIP] + Build for Dev.");
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
    tasks::execute_task_by_id(tasks::CARGO_BUILD).await;

    println!();
    println!("[TIP] + Build at + `target` .");

    // Calculate and display total elapsed time
    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    println!("[TIP] + Done the tasks in {:.2}s.", elapsed_seconds);

    println!("[TIP] + [Task End]");
    println!();
}

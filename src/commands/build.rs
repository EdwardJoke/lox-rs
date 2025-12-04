use crate::projects;
use crate::tasks;

pub fn run() {
    println!();

    // Get project information
    let project = projects::get_or_create_project();

    if project.is_rust_project || project.is_uv_project {
        build_project(&project);
    } else {
        println!("[TIP] + Unknown project type. No build configuration found.");
        println!("[TIP] + [Task End]");
        println!();
    }
}

fn build_project(project: &projects::Project) {
    if project.is_rust_project {
        build_rust_project(project);
    } else if project.is_uv_project {
        build_uv_project(project);
    }
}

fn build_rust_project(_project: &projects::Project) {
    println!("[TIP] + Build for Release.");
    println!();
    println!("[1/3] + Download dependencies");

    // Execute tasks using the new task system
    tasks::execute_task_by_id(tasks::CARGO_UPDATE);
    tasks::execute_task_by_id(tasks::CARGO_FMT);

    println!();
    println!("[2/3] + Check the project");
    tasks::execute_task_by_id(tasks::CARGO_CHECK);

    println!();
    println!("[3/3] + Build the project");
    tasks::execute_task_by_id(tasks::CARGO_BUILD_RELEASE);

    println!();
    println!("[TIP] + Build at + `target` .");
    println!("[TIP] + [Task End]");
    println!();
}

fn build_uv_project(_project: &projects::Project) {
    println!("[TIP] + Build the project.");
    println!();
    println!("[1/3] + Lock the project dependencies");

    // Execute tasks using the new task system
    tasks::execute_task_by_id(tasks::UV_LOCK);

    println!();
    println!("[2/3] + Check and Format the project");
    tasks::execute_task_by_id(tasks::UV_RUFF_CHECK);
    tasks::execute_task_by_id(tasks::UV_RUFF_FORMAT);

    println!();
    println!("[3/3] + Build the project");
    tasks::execute_task_by_id(tasks::UV_BUILD);

    println!();
    println!("[TIP] + Build at + `dist` .");
    println!("[TIP] + [Task End]");
    println!();
}

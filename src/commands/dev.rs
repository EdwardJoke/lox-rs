use crate::projects;
use crate::tasks;

pub fn run() {
    println!();

    // Get project information
    let project = projects::get_or_create_project();

    if project.is_rust_project {
        build_dev_rust_project(&project);
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

fn build_dev_rust_project(project: &projects::Project) {
    println!("[TIP] + Build for Dev.");
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
    tasks::execute_task_by_id(tasks::CARGO_BUILD);

    println!();
    println!("[TIP] + Build at + `target` .");
    println!("[TIP] + [Task End]");
    println!();
}

mod commands;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Check the project informations and the environment
    Doctor,
    /// Build the project in development mode
    Dev,
    /// Build the project in release mode
    Build,
    /// Run the project in development mode
    Dash,
    /// Run the project in release mode
    Run,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Doctor => commands::doctor::run(),
        Commands::Dev => commands::dev::run(),
        Commands::Build => commands::build::run(),
        Commands::Dash => commands::dash::run(),
        Commands::Run => commands::run::run(),
    }
}

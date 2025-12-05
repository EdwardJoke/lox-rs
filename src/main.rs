mod commands;
mod projects;
mod tasks;

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
    Doctor {
        /// Run the command quietly
        #[arg(short = 'q', long = "quiet")]
        quiet: bool,
    },
    /// Build the project in development mode
    Dev,
    /// Build the project in release mode
    Build,
    /// Run the project in development mode
    Dash,
    /// Run the project in release mode
    Run,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Doctor { quiet } => commands::doctor::run(quiet).await,
        Commands::Dev => commands::dev::run().await,
        Commands::Build => commands::build::run().await,
        Commands::Dash => commands::dash::run().await,
        Commands::Run => commands::run::run().await,
    }
}

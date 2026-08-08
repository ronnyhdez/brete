mod commands;
mod model;
mod storage;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "brete", about = "A terminal timer for tracking work hours")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start a timer for a project and tag
    Start {
        project: String,
        tag: Vec<String>,
    },
    /// Stop the running timer and record it
    Stop,
    /// Show the currently running timer, if any
    Status,
    /// Show all recorded sessions
    Log,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { project, tag } => {
            let tag = if tag.is_empty() {
                "general".to_string()
            } else {
                tag.join(" ")
            };
            commands::start(project, tag);
        }
        Command::Stop => commands::stop(),
        Command::Status => commands::status(),
        Command::Log => commands::log(),
    }
}

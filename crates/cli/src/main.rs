mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "spenser")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Start a new review session
    Init { description: String },
    /// Pin a baseline (defaults to HEAD)
    Anchor {
        #[arg(default_value = "HEAD")]
        commit: String,
    },
    /// Diff anchor vs a commit (defaults to HEAD)
    Publish {
        #[arg(default_value = "HEAD")]
        commit: String,
    },
    /// Close the current review session
    Close,
    /// List current review sessions
    List,
    /// Switch to another review session
    Switch { session_id: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Init { description } => commands::init::execute(description),
        Cmd::Anchor { commit } => commands::anchor::execute(commit),
        Cmd::Publish { commit } => commands::publish::execute(commit),
        Cmd::Close {} => commands::close::execute(),
        Cmd::List {} => commands::list::execute(),
        Cmd::Switch { session_id } => commands::switch::execute(session_id),
    }
}
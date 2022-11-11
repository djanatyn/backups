use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command line arguments.
#[derive(Parser, Debug)]
pub struct Args {
    /// Action subcommand.
    #[command(subcommand)]
    pub action: Action,
}

/// What command?
#[derive(Subcommand, Debug)]
pub enum Action {
    Init(Init),
}

/// Initialize a backup repository, and run the first backup.
#[derive(Parser, Debug)]
pub struct Init {
    /// What should the repository be named?
    #[arg(short, long)]
    pub name: String,

    /// What files should be backed up?
    #[arg(short, long)]
    pub path: PathBuf,
}

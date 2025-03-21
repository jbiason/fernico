//! Command line arguments

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    // Path where the database will be saved
    #[arg(default_value = "./", env = "FERNICO_CONFIG_PATH")]
    pub base_dir: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Account management
    #[command(subcommand)]
    Account(crate::accounts::Actions),
}

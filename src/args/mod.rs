//! Command line arguments

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

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
    Account(AccountCommands),
}

#[derive(Subcommand)]
pub enum AccountCommands {
    /// Add a new account
    Add(AccountArgs),

    /// Delete an account
    Del,
}

#[derive(Args)]
pub struct AccountArgs {
    /// The short name for the account, used as an identifier.
    pub short: String,

    /// Full name of the account.
    pub name: String,
}

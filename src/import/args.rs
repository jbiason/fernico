//! Arguments for the import command.

use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub struct ImportArgs {
    /// Name of the file to be imported
    pub filename: PathBuf,
}

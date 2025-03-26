//! Arguments for the "credit" command.

use std::path::PathBuf;

use clap::{Args, ValueEnum};

#[derive(ValueEnum, Clone)]
pub enum Export {
    /// Export accounts
    Accounts,

    /// Exports movement
    Movements,
}

#[derive(Args)]
pub struct ExportArgs {
    /// Content to be exported
    #[arg(value_enum)]
    pub export: Export,

    /// File to save the results
    pub filename: PathBuf,
}

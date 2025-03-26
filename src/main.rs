use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePool;

mod models;
mod tui;

#[derive(Parser)]
struct Cli {
    /// Path for Fernico database.
    #[arg(default_value = "./", env = "FERNICO_CONFIG_PATH")]
    base_dir: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Account management
    #[command(subcommand)]
    Account(tui::account::args::AccountArgs),

    /// Import data from CSV files
    #[command()]
    Import(tui::import::args::ImportArgs),

    /// Export data to CSV files.
    #[command()]
    Export(tui::export::args::ExportArgs),

    /// Add a credit movement
    #[command()]
    Credit(tui::credit::args::CreditArgs),

    /// Add a debit movement
    #[command()]
    Debit(tui::debit::args::DebitArgs),

    /// Transfer values from one account to another
    #[command()]
    Transfer(tui::transfer::args::TransferArgs),
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Cli::parse();

    let options = SqliteConnectOptions::new()
        .filename(args.base_dir.join("fernico.db"))
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    match args.command {
        Commands::Account(action) => tui::account::run(action, &pool).await,
        Commands::Import(import_args) => tui::import::run(&import_args, &pool).await,
        Commands::Export(export_args) => todo!(),
        Commands::Credit(credit_args) => todo!(),
        Commands::Debit(debit_args) => todo!(),
        Commands::Transfer(transfer_args) => todo!(),
    }
}

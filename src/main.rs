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
    Account(tui::account::args::Actions),
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
    }
}

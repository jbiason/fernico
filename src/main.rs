use clap::Parser;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePool;

pub mod accounts;
mod args;
mod import;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = args::Cli::parse();

    let options = SqliteConnectOptions::new()
        .filename("fernico.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    match args.command {
        args::Commands::Account(action) => accounts::run(action, &pool).await,
        args::Commands::Import(args) => import::run(&args, &pool).await,
        args::Commands::Export => todo!(),
        args::Commands::Credit(credit_args) => todo!(),
        args::Commands::Debit(debit_args) => todo!(),
        args::Commands::Transfer(transfer_args) => todo!(),
    }
}

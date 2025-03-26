use args::Actions;
use sqlx::SqlitePool;

use crate::models::account::Account;

pub mod args;

pub async fn run(action: Actions, pool: &SqlitePool) {
    match action {
        Actions::Add(args) => Account::from(args).save(pool).await,
        Actions::Del(args) => {
            let account = Account::get(&args.short, pool).await;
            account.delete(pool).await;
        }
        Actions::List => {
            for account in Account::all(pool).await {
                println!("{:>10} - {}", &account.id, &account.name);
            }
        }
    }
}

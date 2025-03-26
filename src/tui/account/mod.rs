use args::AccountArgs;
use sqlx::SqlitePool;

use crate::models::account::Account;

pub mod args;

pub async fn run(action: AccountArgs, pool: &SqlitePool) {
    match action {
        AccountArgs::Add(args) => Account::from(args).save(pool).await,
        AccountArgs::Del(args) => {
            let account = Account::get(&args.short, pool).await;
            account.delete(pool).await;
        }
        AccountArgs::List => {
            for account in Account::all(pool).await {
                println!("{:>10} - {}", &account.id, &account.name);
            }
        }
    }
}

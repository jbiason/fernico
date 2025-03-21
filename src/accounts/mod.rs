//! Account stuff.

mod args;

pub use args::Actions;

use args::AddArgs;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
struct Account {
    id: String,
    name: String,
}

impl Account {
    pub async fn save(&self, pool: &SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query("INSERT INTO account (id, name) VALUES ($1, $2)")
            .bind(&self.id)
            .bind(&self.name)
            .execute(&mut *conn)
            .await
            .unwrap();
    }

    pub async fn get(id: &str, pool: &SqlitePool) -> Self {
        sqlx::query_as("SELECT id, name from ACCOUNT where id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap()
    }

    pub async fn delete(&self, pool: &SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query("DELETE FROM account WHERE id = %1")
            .bind(&self.id)
            .execute(&mut *conn)
            .await
            .unwrap();
    }

    pub async fn all(pool: &SqlitePool) -> Vec<Account> {
        sqlx::query_as("SELECT id, name FROM account ORDER BY name")
            .fetch_all(pool)
            .await
            .unwrap()
    }
}

impl From<AddArgs> for Account {
    fn from(value: AddArgs) -> Self {
        Self {
            id: value.short,
            name: value.name,
        }
    }
}

pub async fn run(action: args::Actions, pool: &SqlitePool) {
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

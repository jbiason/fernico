//! Account stuff.

mod args;

pub use args::Actions;

use args::AddArgs;
use sqlx::SqlitePool;

pub async fn run(action: args::Actions, pool: &SqlitePool) {
    match action {
        Actions::Add(args) => add(args, pool).await,
        Actions::Del(args) => todo!(),
        Actions::List => list(pool).await,
    }
}

async fn add(values: AddArgs, pool: &SqlitePool) {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query("INSERT INTO account (id, name) VALUES ($1, $2)")
        .bind(&values.short)
        .bind(&values.name)
        .execute(&mut *conn)
        .await
        .unwrap();
}

async fn list(pool: &SqlitePool) {
    #[derive(sqlx::FromRow)]
    struct Acc {
        id: String,
        name: String,
    }

    let results: Vec<Acc> = sqlx::query_as("SELECT id, name FROM account ORDER BY name")
        .fetch_all(pool)
        .await
        .unwrap();

    for record in results {
        println!("{:>10} - {}", &record.id, &record.name);
    }
}

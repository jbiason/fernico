use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
pub struct Account {
    pub id: String,
    pub name: String,
}

impl Account {
    pub fn new(short: &str, name: &str) -> Self {
        Self {
            id: short.to_string(),
            name: name.to_string(),
        }
    }

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


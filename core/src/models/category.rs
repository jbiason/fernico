use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub parent: i32,
    pub name: String,
}

impl Category {
    pub fn new(id: i32, parent: i32, name: &str) -> Self {
        Self {
            id,
            parent,
            name: name.to_string(),
        }
    }

    pub async fn save(&self, pool: &SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query("INSERT INTO category (id, parent, name) VALUES ($1, $2, $3)")
            .bind(&self.id)
            .bind(&self.parent)
            .bind(&self.name)
            .execute(&mut *conn)
            .await
            .unwrap();
    }

    pub async fn search(path: &str, pool: &SqlitePool) -> Option<Category> {
        let mut category: Option<Category> = None;
        let mut walker = path.split('/');
        while let Some(frag) = walker.next() {
            let query = if let Some(parent) = category {
                sqlx::query_as::<_, Category>(
                    "SELECT id, parent, name FROM category where name = $1 AND parent = $2",
                )
                .bind(frag)
                .bind(parent.id)
            } else {
                sqlx::query_as("SELECT id, parent, name FROM category WHERE name = $1").bind(frag)
            };
            if let Ok(record) = query.fetch_one(pool).await {
                category = Some(record)
            } else {
                return None;
            }
        }
        category
    }

    pub async fn delete(&self, pool: &SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query("DELETE FROM category WHERE id = $1")
            .bind(&self.id)
            .execute(&mut *conn)
            .await
            .unwrap();
    }
}

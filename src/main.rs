use sqlx::{query, Executor, MySql, MySqlPool, Pool};

struct MySqlConnection {
    pool: MySqlPool,
}

impl MySqlConnection {
    fn pool(self) -> MySqlPool {
        self.pool
    }
}

#[derive(Clone)]
struct DBConnectionBuilder {
    pool: Option<Pool<MySql>>,
}

impl DBConnectionBuilder {
    fn new() -> Self {
        Self { pool: None }
    }

    async fn establish(mut self, db_url: String) -> Self {
        let pool = MySqlPool::connect(&db_url).await.unwrap();
        self.pool = Some(pool);
        self
    }

    fn build(self) -> MySqlConnection {
        let pool = self.pool.unwrap();

        MySqlConnection { pool }
    }
}

#[tokio::main]
async fn main() {
    let db_url = &std::env::set_var(
        "DATABASE_URL",
        "mysql://root:root@localhost:3306/workforce_db",
    );

    let connection: MySqlConnection = DBConnectionBuilder::new()
        .establish(std::env::var("DATABASE_URL").unwrap())
        .await
        .build();

    let pool = connection.pool();

    let seed_db = sqlx::query!("CREATE DATABASE IF NOT EXISTS workforce_db")
        .execute(&pool)
        .await
        .unwrap();

    println!("{:#?}", seed_db);
}

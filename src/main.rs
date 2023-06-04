use sqlx::{MySql, MySqlPool, Pool};

const DB_URL: &str = "mysql://root:root@localhost:3306/workforce_deb";

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
        Self {
            pool: None,
        }
    }

    async fn pool(mut self, db_url: &str) -> Self {
        let pool = MySqlPool::connect(db_url).await.unwrap();
        self.pool = Some(pool);
        self
    }

    fn build(self) -> MySqlConnection {
        let pool = self.pool.unwrap();

        MySqlConnection {
            pool,
        }
    }
}

#[tokio::main]
async fn main() {
    let connection: MySqlConnection = DBConnectionBuilder::new().pool(DB_URL).await.build();
    let pool = connection.pool();
}

extern crate dotenvy;

use sqlx::{MySql, MySqlPool, Pool};

pub struct MySqlConnection {
    pool: MySqlPool,
}

impl MySqlConnection {
    pub fn pool(self) -> MySqlPool {
        self.pool
    }
}

#[derive(Clone, Default)]
pub struct DBConnectionBuilder {
    pool: Option<Pool<MySql>>,
}

impl DBConnectionBuilder {
    pub async fn establish(mut self, db_url: String) -> Self {
        let pool = MySqlPool::connect(&db_url).await.unwrap();
        self.pool = Some(pool);
        self
    }

    pub fn build(self) -> MySqlConnection {
        let pool = self.pool.unwrap();

        MySqlConnection { pool }
    }
}

pub struct DBConnectionHandler {
    connection: MySqlConnection,
}

impl DBConnectionHandler {
    pub fn new(connection: MySqlConnection) -> Self {
        DBConnectionHandler { connection }
    }
}

use sqlx::{mysql::MySqlQueryResult, MySql, MySqlPool, Pool};

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

struct DBConnectionHandler {
    connection: MySqlConnection,
}

impl DBConnectionHandler {
    fn new(connection: MySqlConnection) -> Self {
        DBConnectionHandler { connection }
    }

    pub async fn seed(self) {
        let create_result = sqlx::query!("CREATE DATABASE IF NOT EXISTS workforce_db")
            .execute(&self.connection.pool())
            .await
            .unwrap();
        println!("{:#?}", create_result);
    }
}

#[tokio::main]
async fn main() {
    let connection: MySqlConnection = DBConnectionBuilder::new()
        .establish(std::env::var("DATABASE_URL").unwrap())
        .await
        .build();

    let connection_handler = DBConnectionHandler::new(connection);
    connection_handler.seed();
}

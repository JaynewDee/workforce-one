extern crate dotenv;
use dotenv::dotenv;

use sqlx::{MySql, MySqlPool, Pool};

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

        let employee_table = sqlx::query!(
            "CREATE TABLE IF NOT EXISTS employee (
            id INT AUTO_INCREMENT PRIMARY KEY,
            first_name VARCHAR(100) NOT NULL,
            last_name VARCHAR(100) NOT NULL,
            hire_date DATE,
            FOREIGN KEY (role_id)
            REFERENCES role(id) 
        );"
        );

        println!("{:#?}", create_result);
    }
}

#[tokio::main]
async fn main() {
    // Tell the compiler to look for a .env file
    dotenv().ok();

    let connection: MySqlConnection = DBConnectionBuilder::new()
        .establish(std::env::var("DATABASE_URL").unwrap())
        .await
        .build();

    let connection_handler = DBConnectionHandler::new(connection);
    connection_handler.seed().await;
}

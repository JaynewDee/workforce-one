extern crate dotenv;

use sqlx::{MySql, MySqlPool, Pool};

pub struct MySqlConnection {
    pool: MySqlPool,
}

impl MySqlConnection {
    fn pool(self) -> MySqlPool {
        self.pool
    }
}

#[derive(Clone)]
pub struct DBConnectionBuilder {
    pool: Option<Pool<MySql>>,
}

impl DBConnectionBuilder {
    pub fn new() -> Self {
        Self { pool: None }
    }

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

    pub async fn seed_all(self) -> Result<(), sqlx::error::Error> {
        let pool = self.connection.pool();

        let create_db_seed = sqlx::query!("CREATE DATABASE IF NOT EXISTS workforce_db");

        let employee_table_seed = sqlx::query!(
            "CREATE TABLE IF NOT EXISTS employee (
                id INT AUTO_INCREMENT PRIMARY KEY,
                first_name VARCHAR(100) NOT NULL,
                last_name VARCHAR(100) NOT NULL,
                role_id INT NOT NULL,
                hire_date DATE,
                FOREIGN KEY (role_id)
                REFERENCES role(id) 
            );"
        );

        let role_table_seed = sqlx::query!(
            "CREATE TABLE IF NOT EXISTS role (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(100) NOT NULL,
                department_id INT NOT NULL,
                FOREIGN KEY (department_id)
                REFERENCES department(id)
            );"
        );

        let department_table_seed = sqlx::query!(
            "CREATE TABLE IF NOT EXISTS department (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(50)
            );"
        );

        let employee_table_result = employee_table_seed.execute(&pool).await?;
        let create_db_result = create_db_seed.execute(&pool).await?;
        let role_table_result = role_table_seed.execute(&pool).await?;
        let department_table_result = department_table_seed.execute(&pool).await?;

        println!("{:#?}", create_db_result);
        println!("{:#?}", employee_table_result);
        println!("{:#?}", role_table_result);
        println!("{:#?}", department_table_result);

        Ok(())
    }
}



extern crate chrono;
extern crate dotenvy;
use dotenvy::dotenv;

mod db;
use db::{seed_all, DBConnectionBuilder, MySqlConnection};
#[tokio::main]
async fn main() -> Result<(), String> {
    // Tell the compiler to look for a .env file
    dotenv().ok();

    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            println!(
                "No DATABASE_URL found within environment.\n
                    Using default dev credentials."
            );
            "mysql://root:root@localhost/workforce_db".to_string()
        }
    };

    let connection: MySqlConnection = DBConnectionBuilder::default()
        .establish(db_url)
        .await
        .build();

    let pool = connection.pool();

    let is_seeded = match seed_all(&pool).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    };

    is_seeded
}

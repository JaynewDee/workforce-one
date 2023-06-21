extern crate chrono;
extern crate dotenvy;

use dotenvy::dotenv;

mod cli;
mod db;
use db::{
    DBConnectionBuilder, Department, Employee, MySqlConnection, Role, WorkforceQueryHandler,
    WorkforceSeeder,
};
use sqlx::Row;

#[derive(Debug)]
enum ViewResult {
    Employees(Vec<Employee>),
    Departments(Vec<Department>),
    Roles(Vec<Role>),
    Invalid(String),
}

async fn handle_view(table_arg: &str, query_handler: WorkforceQueryHandler<'_>) -> ViewResult {
    match table_arg {
        "employees" => {
            let employees = query_handler.view_employees().await.unwrap();
            let collection: Vec<Employee> = employees
                .iter()
                .map(|row| {
                    let first_name = row.get("first_name");
                    let last_name = row.get("last_name");
                    let role_id = row.get("role_id");
                    let manager_id = row.get("manager_id");
                    Employee::new(first_name, last_name, role_id, manager_id)
                })
                .collect();
            ViewResult::Employees(collection)
        }
        "roles" => {
            let roles = query_handler.view_roles().await.unwrap();
            let collection: Vec<Role> = roles
                .iter()
                .map(|row| {
                    let title = row.get("title");
                    let salary = row.get("salary");
                    let department_id = row.get("department_id");

                    Role::new(title, salary, department_id)
                })
                .collect();
            ViewResult::Roles(collection)
        }
        "departments" => {
            let departments = query_handler.view_departments().await.unwrap();
            let collection: Vec<Department> = departments
                .iter()
                .map(|row| {
                    let department_id = row.get("id");
                    let department_name = row.get("name");
                    Department::new(department_id, department_name)
                })
                .collect();
            ViewResult::Departments(collection)
        }
        _ => ViewResult::Invalid(String::from(
            "Invalid Argument passed to `view(v)` flag ... ",
        )),
    }
}

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

    // let is_seeded = match WorkforceSeeder::new(&pool).seed_all().await {
    //     Ok(_) => Ok(()),
    //     Err(e) => Err(e.to_string()),
    // };

    let query_handler = WorkforceQueryHandler::new(&pool);

    let table_arg = cli::main().unwrap();

    let view = handle_view(&table_arg, query_handler).await;

    println!("{:#?}", view);

    // is_seeded
    Ok(())
}

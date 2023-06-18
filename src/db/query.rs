use super::models::{Department, Role};
use super::Employee;
use chrono::prelude::*;
use sqlx::mysql::{MySqlQueryResult, MySqlRow};
use sqlx::MySqlPool;
use sqlx::Row;

pub struct WorkforceQueryHandler<'a> {
    conn_pool: &'a MySqlPool,
}

impl<'a> WorkforceQueryHandler<'a> {
    pub fn new(conn_pool: &'a MySqlPool) -> Self {
        Self { conn_pool }
    }

    pub async fn view_departments(&self) -> Result<MySqlRow, sqlx::Error> {
        let departments_result = sqlx::query("SELECT * FROM department;")
            .fetch_one(self.conn_pool)
            .await;

        departments_result
    }

    pub async fn add_department(
        &self,
        new_department: Department,
    ) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO `department` (name) VALUES (?);")
            .bind(&new_department.name)
            .execute(self.conn_pool)
            .await
    }

    pub async fn department_by_name(
        &self,
        department_name: String,
    ) -> Result<Department, sqlx::error::Error> {
        let result = sqlx::query("SELECT * FROM `department` WHERE name = ?;")
            .bind(&department_name)
            .fetch_one(self.conn_pool)
            .await?;

        Ok(Department::new(result.get("id"), result.get("name")))
    }

    pub async fn view_roles(&self) -> Result<Vec<MySqlRow>, sqlx::Error> {
        sqlx::query("SELECT * FROM role;")
            .fetch_all(self.conn_pool)
            .await
    }

    pub async fn add_role(&self, new_role: Role) -> Result<MySqlQueryResult, sqlx::Error> {
        let insert_role_result =
            sqlx::query("INSERT INTO `role` (title, salary, department_id) VALUES (?, ?, ?);")
                .bind(&new_role.title)
                .bind(&new_role.salary)
                .bind(&new_role.department_id)
                .execute(self.conn_pool)
                .await;

        insert_role_result
    }

    pub async fn role_by_title(&self, role_title: String) {
        let result = sqlx::query("SELECT * FROM `role` WHERE title = ?;")
            .bind(&role_title)
            .execute(self.conn_pool)
            .await;

        println!("{:#?}", result);
    }

    pub async fn view_employees(&self) {
        let result = sqlx::query("SELECT * FROM `employee`;")
            .fetch_all(self.conn_pool)
            .await;

        println!("{:#?}", result)
    }

    pub async fn employee_by_id(&self, employee_id: u32) {
        let result = sqlx::query("SELECT * FROM `employee` WHERE id = ?;")
            .bind(&employee_id)
            .fetch_one(self.conn_pool)
            .await
            .unwrap();

        println!("{:#?}", result);
    }

    pub async fn add_employee(
        &self,
        new_employee: Employee,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let result = sqlx::query("INSERT INTO `employee` (first_name, last_name, role_id, manager_id) VALUES (?, ?, ?, ?);")
            .bind(&new_employee.first_name)
            .bind(&new_employee.last_name)
            .bind(&new_employee.role_id)
            .bind(&new_employee.manager_id)
            .execute(self.conn_pool)
            .await;

        result
    }
}

pub async fn seed_all(conn_pool: &MySqlPool) -> Result<(), sqlx::error::Error> {
    let handler = WorkforceQueryHandler::new(conn_pool);

    let utc: DateTime<Utc> = Utc::now();

    let _ = sqlx::query!("DROP TABLE department;")
        .execute(conn_pool)
        .await;

    let department_table_seed = sqlx::query!(
        "CREATE TABLE IF NOT EXISTS department (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(50) NOT NULL
            );"
    );

    let _ = sqlx::query!("DROP TABLE role;").execute(conn_pool).await;

    let role_table_seed = sqlx::query!(
        "CREATE TABLE IF NOT EXISTS role (
                id INT AUTO_INCREMENT PRIMARY KEY,
                title VARCHAR(100) NOT NULL,
                salary DECIMAL,
                department_id INT NOT NULL,
                FOREIGN KEY (department_id)
                REFERENCES department(id)
            );"
    );

    let _ = sqlx::query!("DROP TABLE employee;")
        .execute(conn_pool)
        .await;

    let employee_table_seed = sqlx::query!(
        "CREATE TABLE IF NOT EXISTS employee (
                id INT AUTO_INCREMENT PRIMARY KEY,
                first_name VARCHAR(100) NOT NULL,
                last_name VARCHAR(100) NOT NULL,
                role_id INT NOT NULL,
                manager_id INT NOT NULL,
                hire_date DATE,
                FOREIGN KEY (role_id)
                REFERENCES role(id),
                FOREIGN KEY (manager_id)
                REFERENCES employee(id)
            );"
    );

    let employee_seeds: Vec<Employee> = vec![
        Employee::new(String::from("Joshua"), String::from("Diehl"), 1, 1, utc),
        Employee::new(String::from("Haylee"), String::from("Diehl"), 1, 1, utc),
        Employee::new(String::from("Jack"), String::from("Ryan"), 1, 1, utc),
    ];

    let role_seeds: Vec<Role> = vec![Role::new(String::from("Agent"), 93_000.00f32, 1)];

    let department_table_result = department_table_seed.execute(conn_pool).await?;
    let role_table_result = role_table_seed.execute(conn_pool).await?;
    let employee_table_result = employee_table_seed.execute(conn_pool).await?;

    let new_department = Department::new(0, String::from("sales"));

    handler.add_department(new_department).await?;
    handler.view_departments().await?;

    for role in role_seeds.into_iter() {
        handler.add_role(role).await?;
    }
    for employee in employee_seeds.into_iter() {
        handler.add_employee(employee).await?;
    }

    println!("{:#?}", department_table_result);
    println!("{:#?}", role_table_result);
    println!("{:#?}", employee_table_result);

    Ok(())
}

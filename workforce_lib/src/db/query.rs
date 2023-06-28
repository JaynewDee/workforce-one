use super::models::{Department, Role};
use super::Employee;
use sqlx::mysql::{MySqlQueryResult, MySqlRow};
use sqlx::MySqlPool;
use sqlx::Row;

pub struct WorkforceQueryHandler<'a> {
    conn_pool: &'a MySqlPool,
}

type QueryResult = Result<MySqlQueryResult, sqlx::Error>;
type RowsResult = Result<Vec<MySqlRow>, sqlx::Error>;

impl<'a> WorkforceQueryHandler<'a> {
    pub fn new(conn_pool: &'a MySqlPool) -> Self {
        Self { conn_pool }
    }

    pub async fn view_departments(&self) -> RowsResult {
        sqlx::query("SELECT * FROM department;")
            .fetch_all(self.conn_pool)
            .await
    }

    pub async fn add_department(&self, new_department: Department) -> QueryResult {
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

    pub async fn view_roles(&self) -> RowsResult {
        sqlx::query("SELECT * FROM role;")
            .fetch_all(self.conn_pool)
            .await
    }

    pub async fn add_role(&self, new_role: Role) -> QueryResult {
        sqlx::query("INSERT INTO `role` (title, salary, department_id) VALUES (?, ?, ?);")
            .bind(&new_role.title)
            .bind(&new_role.salary)
            .bind(&new_role.department_id)
            .execute(self.conn_pool)
            .await
    }

    pub async fn role_by_title(&self, role_title: String) {
        let result = sqlx::query("SELECT * FROM `role` WHERE title = ?;")
            .bind(&role_title)
            .execute(self.conn_pool)
            .await;

        println!("{:#?}", result);
    }

    pub async fn view_employees(&self) -> RowsResult {
        sqlx::query("SELECT * FROM `employee`;")
            .fetch_all(self.conn_pool)
            .await
    }

    pub async fn employee_by_id(&self, employee_id: u32) {
        let result = sqlx::query("SELECT * FROM `employee` WHERE id = ?;")
            .bind(&employee_id)
            .fetch_one(self.conn_pool)
            .await
            .unwrap();

        println!("{:#?}", result);
    }

    pub async fn add_employee(&self, new_employee: Employee) -> QueryResult {
        let result = sqlx::query("INSERT INTO `employee` (first_name, last_name, role_id, manager_id) VALUES (?, ?, ?, ?);")
            .bind(&new_employee.first_name)
            .bind(&new_employee.last_name)
            .bind(&new_employee.role_id)
            .bind(&new_employee.manager_id)
            .execute(self.conn_pool)
            .await;

        result
    }

    pub async fn delete_employee_by_id(&self, id: i32) -> QueryResult {
        sqlx::query("DELETE FROM employee WHERE id = ?;")
            .bind(&id)
            .execute(self.conn_pool)
            .await
    }

    pub async fn update_employee_role(&self, id: i32, new_role: String) -> QueryResult {
        sqlx::query("UPDATE employee SET role = ? WHERE id = ?;")
            .bind(&new_role)
            .bind(&id)
            .execute(self.conn_pool)
            .await
    }
}

pub struct WorkforceSeeder<'a> {
    pool: &'a MySqlPool,
}

impl<'a> WorkforceSeeder<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }

    async fn drop_tables(&self) -> Result<(), sqlx::error::Error> {
        let _ = sqlx::query!("DROP TABLE employee;")
            .execute(self.pool)
            .await
            .unwrap();

        let _ = sqlx::query("DROP TABLE role;")
            .execute(self.pool)
            .await
            .unwrap();

        let _ = sqlx::query("DROP TABLE department;")
            .execute(self.pool)
            .await
            .unwrap();

        Ok(())
    }

    async fn seed_tables(&self) -> Result<(), sqlx::error::Error> {
        let department_table_seed = sqlx::query(
            "CREATE TABLE IF NOT EXISTS department (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(50) NOT NULL
            );",
        );

        let role_table_seed = sqlx::query(
            "CREATE TABLE IF NOT EXISTS role (
                id INT AUTO_INCREMENT PRIMARY KEY,
                title VARCHAR(100) NOT NULL,
                salary FLOAT,
                department_id INT,
                FOREIGN KEY (department_id)
                REFERENCES department(id)
            );",
        );

        let employee_table_seed = sqlx::query(
            "CREATE TABLE IF NOT EXISTS employee (
                id INT AUTO_INCREMENT PRIMARY KEY,
                first_name VARCHAR(100) NOT NULL,
                last_name VARCHAR(100) NOT NULL,
                role_id INT NOT NULL,
                manager_id INT NOT NULL,
                FOREIGN KEY (role_id)
                REFERENCES role(id),
                FOREIGN KEY (manager_id)
                REFERENCES employee(id)
            );",
        );

        department_table_seed.execute(self.pool).await?;
        role_table_seed.execute(self.pool).await?;
        employee_table_seed.execute(self.pool).await?;

        Ok(())
    }

    async fn seed_employee_data(
        &self,
        query_handler: &WorkforceQueryHandler<'a>,
    ) -> Result<(), sqlx::Error> {
        let employee_seeds: Vec<Employee> = vec![
            Employee::new(String::from("Joshua"), String::from("Diehl"), 1, 1),
            Employee::new(String::from("Haylee"), String::from("Diehl"), 1, 1),
            Employee::new(String::from("Jack"), String::from("Ryan"), 1, 1),
        ];

        for employee in employee_seeds.into_iter() {
            query_handler.add_employee(employee).await?;
        }

        Ok(())
    }

    async fn seed_role_data(
        &self,
        query_handler: &WorkforceQueryHandler<'a>,
    ) -> Result<(), sqlx::error::Error> {
        let role_seeds: Vec<Role> = vec![Role::new(String::from("Agent"), 93_000.00f32, 1)];

        for role in role_seeds.into_iter() {
            query_handler.add_role(role).await?;
        }

        Ok(())
    }

    async fn seed_dept_data(
        &self,
        query_handler: &WorkforceQueryHandler<'a>,
    ) -> Result<(), sqlx::Error> {
        let new_department = Department::new(0, String::from("sales"));

        query_handler.add_department(new_department).await?;

        Ok(())
    }

    pub async fn seed_all(&self) -> Result<(), sqlx::error::Error> {
        let handler = WorkforceQueryHandler::new(self.pool);

        // self.drop_tables().await?;
        self.seed_tables().await?;
        self.seed_dept_data(&handler).await?;
        self.seed_role_data(&handler).await?;
        self.seed_employee_data(&handler).await?;

        Ok(())
    }
}

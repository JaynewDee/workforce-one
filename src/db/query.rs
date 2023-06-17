use super::models::{Department, Role};
use sqlx::MySqlPool;
use sqlx::Row;

pub struct WorkforceQueryHandler;

impl WorkforceQueryHandler {
    pub async fn view_departments(conn_pool: &MySqlPool) {
        let departments_result = sqlx::query("SELECT * FROM department;")
            .fetch_one(conn_pool)
            .await
            .unwrap();
        println!("{:#?}", departments_result);
    }

    pub async fn add_department(conn_pool: &MySqlPool, new_department: Department) {
        let insert_dpt_result = sqlx::query("INSERT INTO `department` (name) VALUES (?);")
            .bind(&new_department.name)
            .execute(conn_pool)
            .await;

        println!("{:#?}", insert_dpt_result);
    }

    pub async fn department_by_name(conn_pool: &MySqlPool, department_name: String) {
        let result = sqlx::query("SELECT * FROM `department` WHERE name = ?;")
            .bind(&department_name)
            .fetch_one(conn_pool)
            .await
            .unwrap();

        println!("{:#?}", result.get::<String, &str>("name"));
    }

    pub async fn view_roles(conn_pool: &MySqlPool) {
        let roles_result = sqlx::query("SELECT * FROM role;")
            .fetch_one(conn_pool)
            .await
            .unwrap();

        println!("{:#?}", roles_result);
    }

    pub async fn add_role(conn_pool: &MySqlPool, new_role: Role) {
        let insert_role_result =
            sqlx::query("INSERT INTO `role` (title, salary, department_id) VALUES (?, ?, ?);")
                .bind(&new_role.title)
                .bind(&new_role.salary)
                .bind(&new_role.department_id)
                .execute(conn_pool)
                .await;

        println!("{:#?}", insert_role_result);
    }
}

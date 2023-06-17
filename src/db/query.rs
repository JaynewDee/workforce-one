use sqlx::{MySql, MySqlPool, Pool};

pub struct Department {
    pub id: i32,
    pub name: String,
}
pub struct WorkforceQueryHandler;

impl WorkforceQueryHandler {
    pub async fn view_departments(conn_pool: MySqlPool) {
        let departments_result = sqlx::query!("SELECT * FROM department")
            .fetch_all(&conn_pool)
            .await;
        println!("{:#?}", departments_result);
    }

    pub async fn view_roles(conn_pool: MySqlPool) {
        let roles_result = sqlx::query!("SELECT * FROM role")
            .fetch_all(&conn_pool)
            .await;
        println!("{:#?}", roles_result);
    }

    pub async fn add_department(conn_pool: MySqlPool, new_department: Department) {
        let insert_dpt_result = sqlx::query("INSERT INTO `department` (id, name) VALUES (?, ?);")
            .bind(&new_department.id)
            .bind(&new_department.name)
            .execute(&conn_pool)
            .await;

        println!("{:#?}", insert_dpt_result);
    }
}

use chrono::{DateTime, Utc};
pub struct Department {
    pub id: i32,
    pub name: String,
}

impl Department {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

pub struct Role {
    pub title: String,
    pub salary: f32,
    pub department_id: i32,
}

impl Role {
    pub fn new(title: String, salary: f32, department_id: i32) -> Self {
        Self {
            title,
            salary,
            department_id,
        }
    }
}

pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub role_id: i32,
    pub manager_id: i32,
    pub hire_date: DateTime<Utc>,
}

impl Employee {
    pub fn new(
        first_name: String,
        last_name: String,
        role_id: i32,
        manager_id: i32,
        hire_date: DateTime<Utc>,
    ) -> Self {
        Self {
            first_name,
            last_name,
            role_id,
            manager_id,
            hire_date,
        }
    }
}

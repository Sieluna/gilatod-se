use serde::{Deserialize, Serialize};

use crate::configs::DatabaseScheme;
use crate::entities::Table;

#[derive(sqlx::FromRow, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct UserTable;

impl Table for UserTable {
    fn name(&self) -> &'static str {
        "users"
    }

    fn create(&self, scheme: &DatabaseScheme) -> String {
        let id_type = match scheme {
            DatabaseScheme::POSTGRES => "INT GENERATED BY DEFAULT AS IDENTITY",
            DatabaseScheme::SQLITE => "INTEGER PRIMARY KEY AUTOINCREMENT",
            DatabaseScheme::MYSQL => "INT AUTO_INCREMENT PRIMARY KEY",
        };

        let text_type = "VARCHAR(255)";

        format!(
            "CREATE TABLE IF NOT EXISTS {} (\
                id {id_type}, \
                username {text_type} NOT NULL UNIQUE, \
                email {text_type} NOT NULL UNIQUE, \
                password {text_type} NOT NULL);",
            self.name()
        )
    }

    fn dispose(&self) -> String {
        format!("DROP TABLE IF EXISTS {};", self.name())
    }

    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }
}

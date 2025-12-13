use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: -1,
            name: "".to_string(),
            email: "".to_string(),
        }
    }
}

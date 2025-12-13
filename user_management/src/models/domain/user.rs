use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

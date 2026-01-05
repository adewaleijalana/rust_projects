use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    created_at: String,
}

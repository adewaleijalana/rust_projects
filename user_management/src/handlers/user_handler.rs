use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::models::domain::user::User;

pub async fn create_user() -> &'static str {
    "Welcome to the User Management API"
}

pub async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_user() -> &'static str {
    "Welcome to the User Management API"
}

pub async fn update_user() -> &'static str {
    "Welcome to the User Management API"
}

pub async fn delete_user() -> &'static str {
    "Welcome to the User Management API"
}

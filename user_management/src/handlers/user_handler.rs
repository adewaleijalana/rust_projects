use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::models::{domain::user::User, exchanges::requests::user_payload::UserRequest};

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<User>, StatusCode> {
    let sql = r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email
    "#;

    sqlx::query_as::<_, User>(&sql)
        .bind(&payload.get_name())
        .bind(&payload.get_email())
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_user(State(pool): State<PgPool>) -> Result<Json<User>, StatusCode> {
    Ok(Json(User::new()))
}

pub async fn update_user() -> Result<Json<User>, StatusCode> {
    Ok(Json(User::new()))
}

pub async fn delete_user() -> &'static str {
    "Welcome to the User Management API"
}

use axum::{
    Router, extract::{Path, State}, http::StatusCode, routing::{Route, delete, get, put, post}
};
use sqlx::{postgres::PgPoolOptions};

use std::env;

mod models;

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connecto to db");

    sqlx::migrate!().run(&db_pool).await.expect("Migration failed")

    let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user).get(list_users))
    .route("/user/{id}", get(get_user).put(update_user).delete(delete_user))
    .with_state(&db_pool);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8181").await.unwrap();
    println!("Server running on port 8181");

    axum::serve(listener, app).await.unwrap();
}

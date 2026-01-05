use diesel::prelude::*;
use rocket::{
    response::status,
    serde::json::{Value, json},
};
// use serde_json::json;

use crate::{auth::basic_auth::BasicAuth, db::db_conn::DBConn};
use crate::{models::user::User, schema::users};

#[get("/users")]
pub async fn get_users(auth: BasicAuth, db: DBConn) -> Value {
    db.run(|conn| {
        let users = users::table
            .order(users::id.asc())
            .limit(1000)
            .load::<User>(conn)
            .expect("Error fetching users");

        json!(users)
    })
    .await
}

#[get("/users/<id>")]
pub fn get_users_by_id(id: u32, auth: BasicAuth, db: DBConn) -> Value {
    json!([{"id": 1, "name": "Tester 1"}])
}

#[post("/users", format = "json")]
pub fn add_user(auth: BasicAuth, db: DBConn) -> Value {
    json!([{"id": 3, "name": "Tester 1"}])
}

#[put("/users/<id>", format = "json")]
pub fn update_user(id: u32, auth: BasicAuth, db: DBConn) -> Value {
    json!([{"id": id, "name": "Tester 1"}])
}

#[delete("/users/<id>")]
pub fn delete_users_by_id(id: u32, auth: BasicAuth, db: DBConn) -> status::NoContent {
    status::NoContent
}

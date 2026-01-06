use diesel::prelude::*;
use rocket::{
    response::status,
    serde::json::{Json, Value, json},
};


use crate::{
    auth::basic_auth::BasicAuth, db::db_conn::DBConn,
    exchanges::requests::user_request::UserRequest,
};
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
pub async fn get_users_by_id(id: i32, auth: BasicAuth, db: DBConn) -> Value {

    db.run(move |conn| {
        let user = users::table
            .filter(users::id.eq(id))
            .first::<User>(conn)
            .expect("Error fetching user using id");

        json!(user)
    }).await
}

#[post("/users", format = "json", data = "<new_user_request>")]
pub async fn add_user(auth: BasicAuth, new_user_request: Json<UserRequest>, db: DBConn) -> Value {
    db.run(|conn| {
        let result = diesel::insert_into(users::table)
            .values(new_user_request.into_inner())
            .execute(conn)
            .expect("Error creating user");

        json!(result)
    }).await
}

#[put("/users/<id>", format = "json")]
pub fn update_user(id: i32, auth: BasicAuth, db: DBConn) -> Value {
    json!([{"id": id, "name": "Tester 1"}])
}

#[delete("/users/<id>")]
pub fn delete_users_by_id(id: i32, auth: BasicAuth, db: DBConn) -> status::NoContent {
    status::NoContent
}

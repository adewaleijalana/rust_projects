use diesel::prelude::*;
use rocket::{
    execute,
    http::Status,
    response::status::{self, Custom},
    serde::json::{Json, Value, json},
};

use crate::{
    auth::basic_auth::BasicAuth,
    db::db_conn::DBConn,
    exchanges::requests::{update_user_request::UpdateUserRequest, user_request::UserRequest},
    repositories::{
        base_create_repository::BaseCreateRepository,
        base_repository::BaseRepository,
        user_repository::{self, UserRepository},
    },
};
use crate::{models::user::User, schema::users};

#[get("/users")]
pub async fn get_users(auth: BasicAuth, db: DBConn) -> Result<Value, Custom<Value>> {
    // db.run(|conn| {
    //     let users = users::table
    //         .order(users::id.asc())
    //         .limit(1000)
    //         .load::<User>(conn)
    //         .expect("Error fetching users");

    //     json!(users)
    // })
    // .await

    db.run(|conn| {
        UserRepository::find_all(conn, 100)
            .map(|users| json!(users))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/users/<id>")]
pub async fn get_users_by_id(id: i32, auth: BasicAuth, db: DBConn) -> Result<Value, Custom<Value>> {
    // db.run(move |conn| {
    //     let user = users::table
    //         .filter(users::id.eq(id))
    //         .first::<User>(conn)
    //         .expect("Error fetching user using id");

    //     json!(user)
    // })
    // .await

    db.run(move |conn| {
        UserRepository::find_by_id(id, conn)
            .map(|user| json!(user))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/users", format = "json", data = "<new_user_request>")]
pub async fn add_user(
    auth: BasicAuth,
    new_user_request: Json<UserRequest>,
    db: DBConn,
) -> Result<Value, Custom<Value>> {
    // db.run(|conn| {
    //     let result = diesel::insert_into(users::table)
    //         .values(new_user_request.into_inner())
    //         .execute(conn)
    //         .expect("Error creating user");

    //     json!(result)
    // })
    // .await

    db.run(|conn| {
        UserRepository::save(conn, new_user_request.into_inner())
            .map(|user| json!(user))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/users/<id>", format = "json", data = "<update_user_request>")]
pub async fn update_user(
    id: i32,
    auth: BasicAuth,
    update_user_request: Json<UpdateUserRequest>,
    db: DBConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(update_user_request.into_inner())
            .execute(conn)
            .map(|affected_rows| json!(affected_rows))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/users/<id>")]
pub async fn delete_users_by_id(id: i32, auth: BasicAuth, db: DBConn) -> status::NoContent {
    db.run(move |conn| {
        diesel::delete(users::table)
            .filter(users::id.eq(id))
            .execute(conn)
            .expect("Error deleting record")
    })
    .await;
    status::NoContent
}

use diesel::prelude::*;
use rocket::{
    execute,
    response::status,
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
pub async fn get_users(auth: BasicAuth, db: DBConn) -> Value {
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
        let users = UserRepository::find_all(conn, 100).expect("Error fetching users");
        json!(users)
    })
    .await
}

#[get("/users/<id>")]
pub async fn get_users_by_id(id: i32, auth: BasicAuth, db: DBConn) -> Value {
    // db.run(move |conn| {
    //     let user = users::table
    //         .filter(users::id.eq(id))
    //         .first::<User>(conn)
    //         .expect("Error fetching user using id");

    //     json!(user)
    // })
    // .await

    db.run(move |conn| {
        let user = UserRepository::find_by_id(id, conn).expect("Error fetching user using id");

        json!(user)
    })
    .await
}

#[post("/users", format = "json", data = "<new_user_request>")]
pub async fn add_user(auth: BasicAuth, new_user_request: Json<UserRequest>, db: DBConn) -> Value {
    // db.run(|conn| {
    //     let result = diesel::insert_into(users::table)
    //         .values(new_user_request.into_inner())
    //         .execute(conn)
    //         .expect("Error creating user");

    //     json!(result)
    // })
    // .await

    db.run(|conn| {
        let result =
            UserRepository::save(conn, new_user_request.into_inner()).expect("Error creating user");
            
        json!(result)
    })
    .await
}

#[put("/users/<id>", format = "json", data = "<update_user_request>")]
pub async fn update_user(
    id: i32,
    auth: BasicAuth,
    update_user_request: Json<UpdateUserRequest>,
    db: DBConn,
) -> Value {
    db.run(move |conn| {
        let result = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(update_user_request.into_inner())
            .execute(conn)
            .expect("Error updating user");

        json!(result)
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

#![allow(unused)]

#[macro_use]
extern crate rocket;

mod routes;

use routes::users::users_routes::{
    add_user, delete_users_by_id, get_users, get_users_by_id, update_user,
};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![index])
        .mount(
            "/",
            routes![
                get_users,
                get_users_by_id,
                add_user,
                update_user,
                delete_users_by_id
            ],
        )
        .launch()
        .await;
}

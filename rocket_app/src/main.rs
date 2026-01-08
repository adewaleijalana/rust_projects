#![allow(unused)]

#[macro_use]
extern crate rocket;

mod auth;
mod db;
mod exchanges;
mod models;
mod repositories;
mod routes;
mod schema;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use routes::users::users_routes::{
    add_user, delete_users_by_id, get_users, get_users_by_id, update_user,
};

use routes::errors::errors_handlers::{not_found, un_authorized, unprocessable_entity};

use crate::db::db_conn::DBConn;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DBConn::get_one(&rocket)
        .await
        .expect("Unable to acquire connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
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
        .register(
            "/",
            catchers![not_found, un_authorized, unprocessable_entity],
        )
        .attach(DBConn::fairing())
        .attach(AdHoc::on_ignite("DB Migration", run_db_migrations))
        .launch()
        .await;
}

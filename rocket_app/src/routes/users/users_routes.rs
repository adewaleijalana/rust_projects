use rocket::{
    response::status,
    serde::json::{Value, json},
};

#[get("/users")]
pub fn get_users() -> Value {
    json!([{"id": 1, "name": "Tester 1"}, {"id": 2, "name": "Tester 2"}])
}

#[get("/users/<id>")]
pub fn get_users_by_id(id: u32) -> Value {
    json!([{"id": 1, "name": "Tester 1"}])
}

#[post("/users", format = "json")]
pub fn add_user() -> Value {
    json!([{"id": 3, "name": "Tester 1"}])
}

#[put("/users/<id>", format = "json")]
pub fn update_user(id: u32) -> Value {
    json!([{"id": id, "name": "Tester 1"}])
}

#[delete("/users/<id>")]
pub fn delete_users_by_id(id: u32) -> status::NoContent {
    status::NoContent
}

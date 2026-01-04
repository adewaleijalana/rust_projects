use rocket::{
    Request,
    serde::json::{Value, json},
};

#[catch(404)]
pub fn not_found(_req: &Request) -> Value {
    json!([{"message": "Not Found"}])
}

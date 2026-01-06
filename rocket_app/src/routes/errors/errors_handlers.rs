use rocket::{
    Request,
    serde::json::{Value, json},
};

#[catch(404)]
pub fn not_found(_req: &Request) -> Value {
    json!({"message": "Not Found"})
}

#[catch(401)]
pub fn un_authorized(_req: &Request) -> Value {
    json!({"message": "Unauthorized"})
}

#[catch(422)]
pub fn  unprocessable_entity(_req: &Request) -> Value {
    json!({"message": "Unauthorized"})
}

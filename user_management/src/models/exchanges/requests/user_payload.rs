use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRequest{
  name: String,
  email: String
}
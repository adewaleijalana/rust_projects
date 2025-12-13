use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRequest {
    name: String,
    email: String,
}

impl UserRequest {
    pub fn get_name(&self) -> String {
      self.name.clone()
    }

    pub fn get_email(&self) -> String {
      self.email.clone()
    }
}

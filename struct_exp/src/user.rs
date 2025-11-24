#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct User {
  pub first_name: String,
  pub last_name: String,
  pub age: u8
}

impl User {
  pub fn default_user() -> Self{
    User{
      first_name: String::from("Rose"),
      last_name: String::from("Dewale"),
      age: 30,
    }
  }

  pub fn new_user(first_name: String, last_name: String, age: u8) -> Self{
    User{
      first_name,
      last_name,
      age,
    }
  }

  pub fn get_first_name(&self) -> &str {
    &self.first_name
  }

  pub fn get_age(&self) -> u8 {
    self.age
  }
}
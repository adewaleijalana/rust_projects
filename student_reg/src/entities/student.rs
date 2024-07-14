#[allow(dead_code)]
pub struct Student {
  first_name: String,
  last_name: String
}

impl Student {
  pub fn new(first_name: String, last_name: String) -> Self {
    Student {
      first_name,
      last_name
    }
  }

  pub fn get_first_name(&self) -> &String {
      &self.first_name
  }
}
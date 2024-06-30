
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Student {
  first_name: String,
  last_name: String
}

impl Student {
  pub fn new (first_name: &str, last_name: &str) -> Self {
    Student {
      first_name: first_name.to_string(),
      last_name: last_name.to_string()
    }
  }

  pub fn get_first_name(&self) -> String {
    self.first_name.clone()
  }

  pub fn get_last_name(&self) -> String {
    self.last_name.clone()
  }
}
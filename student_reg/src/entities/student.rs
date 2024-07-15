#[allow(dead_code)]
#[derive(Debug)]
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

  pub fn set_first_name(&mut self, first_name: String) {
      self.first_name = first_name;
  }
}
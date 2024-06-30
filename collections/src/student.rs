#[derive(Debug)]
pub struct Student {
  first_name: String,
  last_name: String
}

impl Student {
    pub fn new (first_name: String, last_name: String) -> Self {
      Student {
        first_name, last_name
      }
    }

    pub fn get_first_name(&self) -> &String {
        return &self.first_name;
    }

    pub fn get_last_name(&self) -> &String {
        return &self.last_name;
    }
}
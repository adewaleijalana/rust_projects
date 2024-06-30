use crate::Student;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Teacher {
  first_name: String,
  last_name: String
}

#[allow(dead_code)]
impl Teacher {
  fn new (first_name: &str, last_name: &str) -> Teacher {
    Teacher {
      first_name: first_name.to_string(),
      last_name: last_name.to_string()
    }
  }
}

impl From<Student> for Teacher {
  fn from(student: Student) -> Teacher {
    Teacher {
      first_name: format!("{}{}", "Teacher: ", student.get_first_name()),
      last_name: format!("{}{}", "Teacher: ", student.get_last_name())
    }
  }
}
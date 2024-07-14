pub struct Course {
  course_name: String
}

impl Course {
  pub fn new(course_name: String) -> Self {
    Course {
      course_name,
    }
  }

  pub fn get_course_name(&self) -> &String {
      &self.course_name
  }
}
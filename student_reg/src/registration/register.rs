use crate::entities::{course::Course, student::Student};
pub struct Register<'a> {
  student: &'a Student,
  course: &'a Course
}

impl<'a> Register<'a> {
  pub fn registration(student: &'a Student, course: &'a Course) -> Self {
    Register { student, course}      
  }
    pub fn register(&self){
      println!("Registering the student: {} for the course: {}", self.student.get_first_name(), self.course.get_course_name())
    }
}
use std::{
    error::Error,
    fs,
    io::{BufWriter, Write},
};

use crate::entities::{course::Course, student::Student};
pub struct Register<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Register<'a> {
    pub fn registration(student: &'a Student, course: &'a Course) -> Self {
        Register { student, course }
    }
    pub fn register(&self) -> Result<(), Box<dyn Error>> {
        println!(
            "Registering the student: {} for the course: {}",
            self.student.get_first_name(),
            self.course.get_course_name()
        );
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("student_reg.txt")?;
        let mut buff = BufWriter::new(file);
        let _ = writeln!(
            buff,
            "{}",
            format!(
                "{},{}",
                self.student.get_first_name(),
                self.course.get_course_name()
            )
        );
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register() {
        let student = Student::new("Rose".to_string(), "Dewale".to_string());
        let course = Course::new("Physiology".to_string());
        let registration = Register::registration(&student, &course);

        let reg = registration.register();

        assert!(reg.is_ok());
    }
}

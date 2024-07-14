mod registration;
mod entities;

use entities::{student::Student, course::Course};
use registration::register::Register;
fn main() {
    let student = Student::new("Rose".to_string(), "Dewale".to_string());
    let course = Course::new("Physiology".to_string());
    let registration = Register::registration(&student, &course);

    registration.register();
}

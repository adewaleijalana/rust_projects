mod registration;
mod entities;

use entities::{student::Student, course::Course};
use registration::register::Register;
fn main() {
    let student = Student::new("Rose".to_string(), "Dewale".to_string());

    let mut student2 = Student::new("Rose".to_string(), "Dewale".to_string());
    student2.set_first_name(String::from("Titi"));
    println!("{:?}", student2);
    let course = Course::new("Physiology".to_string());
    let registration = Register::registration(&student, &course);

    let _ = registration.register();
}

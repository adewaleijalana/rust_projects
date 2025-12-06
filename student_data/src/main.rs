#![allow(unused_imports, unused_mut, dead_code)]

mod students;

use students::read_student_file;
fn main() {
    let students = read_student_file();
    match students {
        Ok(students) => println!("{students:#?}"),
        Err(e) => println!("{e:#?}")
    }
}

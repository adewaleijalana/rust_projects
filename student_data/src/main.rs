#![allow(unused_imports, unused_mut, dead_code)]

mod students;

use std::io;

use students::read_student_file;
fn main() -> io::Result<()>{
    let students = read_student_file();
    let students = match students {
        Ok(students) => students,
        Err(e) => {
            println!("{e:#?}");
            return Err(e);
        }
    };

    println!("stedunt lists {students:#?}");

    Ok(())
}

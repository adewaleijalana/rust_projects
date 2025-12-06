use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error};

use crate::students::student_data::Student;

pub fn read_student_file() -> io::Result<Vec<Student>> {
    let mut students = Vec::<Student>::new();

    let file = File::open("student_records.txt")?;

    let reader = BufReader::new(file);

    for line_result in reader.lines().skip(1) {
        let line = line_result?; // Handle potential errors reading a line
        // if let Some(student) = Student::parse_student(line) {
        //     students.push(student);
        // }
        let std = Student::parse_student(line)
            .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidData, "bad line"))?;
        // println!("{}", line);
        students.push(std);
    }
    Ok(students)
}

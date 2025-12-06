use std::io::{self, BufRead, BufReader};
use std::fs::{self, File};

use crate::students::student_data::Student;

pub fn read_student_file() -> io::Result<Vec<Student>>{

  let mut students = Vec::<Student>::new();

  let file = File::open("student_records.txt")?;

  let reader = BufReader::new(file);

  for line_result in reader.lines().skip(1) {
        let line = line_result?;
        if let Some(student) = Student::parse_student(line){
          students.push(student);
        }
    }
  Ok(students)
    
}
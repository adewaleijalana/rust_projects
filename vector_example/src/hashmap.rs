use std::collections::HashMap;
use crate::Student;

pub fn _test_hashmap(){
  let mut student_scores: HashMap<&str, i32> = HashMap::new();
  student_scores.insert("Neriah", 99);
  student_scores.insert("Jazmine", 99);
  student_scores.insert("Rose", 95);
  student_scores.insert("Dewale", 99);

  for (key, value) in  &student_scores {
      println!("Key: {}; Value: {}", key, value)
  }

  println!("");

  student_scores.entry("Titi")
  // .and_modify(|e| *e += 5)
  .or_insert(65);



  for (key, value) in  student_scores {
      println!("Key: {}; Value: {}", key, value)
  }
}

#[allow(dead_code)]
pub fn student_hashmap(){
  let students: HashMap<Student, i32> = HashMap::from([
    (Student::new("Rose", "Dewale"), 55),
    (Student::new("Neriah", "Ijalana"), 55),
  ]);

  for (key, value) in  students {
      println!("Key: {:?}; Value: {}", key, value)
  }
}
mod student;

use std::collections::HashMap;

use student::Student;
fn main() {
   let students = vec![
    Student::new(String::from("Rose"), String::from("Ijalana")),
    Student::new(String::from("Dewale"), String::from("Rose")),
    Student::new(String::from("Kay"), String::from("Jay")),
   ];

   for student in  &students{
       println!("First name: {}; Last name: {}", &student.get_first_name(), &student.get_last_name());
   }

   println!("{:?}", students);

   let mut scores = HashMap::new();
   scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 40);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

}

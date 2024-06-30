#[allow(unused_variables, unused_imports)]
mod hashmap;
mod school;
// use school::student::Student;
// use school::teacher::Teacher;

use school::{teacher::Teacher, student::Student};

#[allow(unused_variables, unused_imports)]
fn main() {
    // // let arr: [u8; 3] = [1, 2, 3];

    // let mut v = Vec::from([1, 2, 3]);

    // for i in 0..5 {
    //     println!("{:?}", v.get(i))
    // }

    // for i in 0..5 {
    //     match v.get(i) {
    //         Some(e) => v[i] = e + 1,
    //         None => v.push(i + 2)
    //     }
    // }

    // assert_eq!(v, vec![2, 3, 4, 5, 6]);
    // println!("Successs")
    // test_hashmap();
    // student_hashmap();
    let teacher_from_student = Teacher::from(Student::new("Adewale", "Ijalana"));
    println!("Teacher create from Student: {:?}", teacher_from_student)

}

#[allow(dead_code, unused_variables)]
fn is_vec(v: Vec<u8>){}
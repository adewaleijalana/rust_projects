#![allow(unused_imports, unused)]
mod lifetime;

use lifetime::functions::*;
use lifetime::{dentist_appointment::DentistAppointment, functions, test_life_time};

fn main() {
    //    test_life_time::lifetime_test_error();
    //    test_life_time::lifetime_test_2()

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest::longest(string1.as_str(), string2);
    // println!("The longest string is {result}");

    // let dentist_appointment = DentistAppointment::new(String::from("Dr. Oluwa"));
    // let result = dentist_appointment.book("10:00 am", "12:00 pm");
    // println!("{result}")

    // let len = double_the_length::<i32>(&vec![1, 2, 3]);

    // let len = double_the_length::<&str>(&vec!["1", "2", "3"]);

    // println!("{}", find_string_that_has_content("programming", "dining", "gram"));

    println!("{:?}", last_two(&["1", "2", "3", "7", "8", "9", "10"]));

    println!("{:?}", last_two(&[1, 2, 3, 7, 8, 9, 10]));
}

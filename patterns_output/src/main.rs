#![allow(unused, dead_code)]
mod arrays_probs;
mod entities;
mod patterns;
mod switch_mode;

use arrays_probs::arrays::{arrays_test, divsion, slice_array, sort_array, word_to_bytes};
use entities::user::{self, User};
use patterns::{match_struct, pattern_match, Language, Point};
use std::{error::Error, io};
use switch_mode::{pattern_printing, space_printing};

// fn main() -> Result<(), Box<dyn Error>>{

//     // sort_array();
//     // pattern_printing();
//     // space_printing();
//     // println!("Sum of digits in doubled number: {}", double_digit(&8));
//     // char();
//     // switch_mode();

//     let result = divsion(25, 0)?;
//     println!("{result}");

//     Ok(())
// }

fn main() {
    // let lang = Language::ENGLISH(String::from("Fluently"));
    // pattern_match(&lang);

    // let tp = (1, "Test", Point::new(10, 100));
    // let (a, b, c) = tp;

    // let p = c;

    // // let point = Point::new(0, 7);
    // // let point = Point::new(10, 0);
    // let point = Point::new(10, 100);
    // let Point { x:a, y:b } = point;
    // println!("{a} and {b}")
    // // match_struct(&point);

    // let s = String::from(" ");
    // word_to_bytes(&s);

    // let mut  arr = [1, 2, 3, 4, 5];
    // slice_array(&mut arr);

    let mut user = User::new(
        1,
        String::from("Test"),
        String::from("test@email.com"),
        String::from("password"),
    );

    use_user(&mut user);

    println!("user name is: {}", user.get_name());
}


fn use_user(user: &mut User) {
    println!("user name in method call: {}", user.get_name());
    user.set_name("New Name");
}
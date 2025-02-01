#![allow(unused, dead_code)]
mod switch_mode;
mod arrays_probs;
mod patterns;


use std::{error::Error, io};
use switch_mode::{pattern_printing, space_printing};
use arrays_probs::arrays::{arrays_test, divsion, sort_array};
use patterns::{pattern_match, Language, Point, match_struct};

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
    let lang = Language::ENGLISH(String::from("Fluently"));
    pattern_match(&lang);

    let tp = (1, "Test", Point::new(10, 100));
    let (a, b, c) = tp;

    let p = c;

    // let point = Point::new(0, 7);
    // let point = Point::new(10, 0);
    let point = Point::new(10, 100);
    let Point { x:a, y:b } = point;
    println!("{a} and {b}")
    // match_struct(&point);
}
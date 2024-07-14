use std::fmt::Display;

fn main() {
    //1
//    let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result}");

//2
    // let string1 = String::from("long string is long");
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is: {result}");
    // }
//3

    // let string1 = String::from("long string is long");
    // let result;

    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is: {result}");
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T : Display{
    print!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

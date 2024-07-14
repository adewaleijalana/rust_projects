use std::io;

fn main() {
    println!("This program is to take user input!");

    println!("Enter your name");

    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read name");

    let name = name.trim();

    println!("Enter your age ");
    let mut age = String::new();

    io::stdin()
    .read_line(&mut age)
    .expect("error reading age");

    println!("Your name is {name} and age is {age}")

}

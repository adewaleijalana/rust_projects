mod user;
mod car;
use user::User;
use car::Car;

use std::{collections::BinaryHeap, fs::File};

fn main() {
    // // let user = User::default_user();
    // let user2 = User::new_user(String::from("Rose"), String::from("Adewale"), 30);
    // let user3 = User{
    //     first_name: String::from("Jazmine"),
    //     ..user2.clone() // Cloning user2 to inherit its last name and age
    // };

    // let mut user_queue = BinaryHeap::new();
    // user_queue.push(&user2);
    // user_queue.push(&user3);

    // // println!("{:?}", user2);
    // println!("first name is {}", user3.get_first_name());

    // let mut car1 = Car::new(String::from("30 MPG"), String::from("Blue"), 150);
    // car1.set_mpg(String::from("35 MPG"));
    // car1.set_color(String::from("Green"));
    // car1.set_top_speed(160);

    // println!("Car details:");
    // println!("MPG: {}", car1.mpg);
    // println!("Color: {}", car1.color);
    // println!("Top Speed: {}", car1.top_speed);

    // let file = File::open("user_data.txt");

    let nums = vec![1, 3, 5, 7, 9];
    let multiplied: Vec<i32> = nums.iter().map(|x| x * 10).collect();
    println!("Multiplied numbers: {:?}", multiplied);

    println!("Original numbers: {:?}", nums);
}
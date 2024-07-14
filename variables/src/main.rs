use std::{array, io};

mod constants;
fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // println!("test constant: {}", constants::TEST_NAME)

    // for i in 1..=12 {
    //     println!("5 * {i} = {}", 5 * i)
    // }

    println!("This program print out Multipication table as chosen by the user");

    loop {
        let mut limit = String::new();
        let mut multiplier = String::new();

        println!("Please enter the limit: ");

        io::stdin()
            .read_line(&mut limit)
            .expect("Error reading the limit");

        println!("limit: {limit}");

        let limit: u32 = match limit.trim().parse() {
            Ok(limit) => limit,
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };

        println!("Please enter the multiplier: ");

        io::stdin()
            .read_line(&mut multiplier)
            .expect("Error reading the limit");

        let multiplier: u32 = match multiplier.trim().parse() {
            Ok(multiplier) => multiplier,
            Err(_) => continue,
        };

        for i in 1..=limit {
            println!("{multiplier} * {i} = {}", multiplier * i);
        }

        println!("Do you want to run other multiplication? Y/N");

        let mut decision = String::new();

        io::stdin()
            .read_line(&mut decision)
            .expect("Please choose an option (Y/N)");

        let decision = decision.trim();

        if decision.eq_ignore_ascii_case("n") {
            break;
        }
    }
}

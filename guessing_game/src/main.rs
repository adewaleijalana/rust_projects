
mod guess;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use guess::Guess;

// fn main() -> Result<(), std::io::Error> {
//     println!("Enter a number: ");

//     let mut input  = String::new();

//     let result = match io::stdin().read_line(&mut input) {
//         Ok(_) => input,
//         Err(e) => return Err(e)
//     };

//     println!("Inputed value: {}", result);

//     Ok(())

// }

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rngs::OsRng.gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Enter a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess).value();

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

use colored::Colorize;
use rand::rng;
use rand::seq::IndexedRandom;
use std::io::{self, Write};

pub fn banner() {
    let banner = r"
     _       ______  ____  ____  __    ______
    | |     / / __ \/ __ \/ __ \/ /   / ____/
    | | /| / / / / / /_/ / / / / /   / __/   
    | |/ |/ / /_/ / _, _/ /_/ / /___/ /___   
    |__/|__/\____/_/ |_/_____/_____/_____/";

    println!("{}", format!("{banner}").green());
    println!("");
    println!("");

    play();
    io::stdout().flush().unwrap();
}

pub fn play() {
    let mut target_word = "trait";

    let words: [&str; 25] = [
        "apple", "cloud", "brick", "flame", "river", "stone", "grain", "track", "light", "sauce",
        "frame", "pivot", "blaze", "shore", "crisp", "drift", "grand", "wrist", "maple", "storm",
        "quilt", "phase", "spark", "honey", "rogue",
    ];

    let mut rng = rng();

    if let Some(word) = words.choose(&mut rng) {
        // println!("Random word: {word}");
        target_word = word;
    }

    let input = io::stdin();
    for _ in 1..=6 {
        println!("Guess a five-letter world");
        let mut guess = String::new();
        input.read_line(&mut guess).expect("Enter a word");
        let guess = guess.trim();
        // println!("{}", guess);

        let zip_result = target_word.chars().zip(guess.chars().take(5));
        for (target_char, guess_char) in zip_result {
            if target_char == guess_char {
                print!(" {} ", format!(" {guess_char} ").on_green())
            } else if target_word.contains(guess_char) {
                print!(" {} ", format!(" {guess_char} ").on_yellow())
            } else {
                print!(" {} ", format!(" {guess_char} ").on_black())
            }
        }

        println!("");

        if target_word == guess {
            println!("You guessed right!!!");
            break;
        }
    }
}

pub fn while_loop(end: i32) {
    let mut counter  = 0;

    // while true {
    //     println!("counter is: {counter}");
    //     if counter >= end{
    //         break;
    //     }
    //     counter += 1;
    // } 

    loop {
        println!("counter is: {counter}");
        if counter >= end{
            break;
        }
        counter += 1;
    };
}

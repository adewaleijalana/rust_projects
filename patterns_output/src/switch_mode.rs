use core::num;
use std::io;

pub fn pattern_printing() {
    for i in 1..=6 {
        for _ in 1..=(5 - i) {
            print!("#");
        }
        println!();
    }
}

pub fn space_printing() {
    for i in 1..=6 {
        for _ in 1..= i {
            print!(" ");
        }

        for _ in 1..= (5 - i) {
            print!("#");
        }

        for _ in 1..= (5 - i) {
            print!("#");
        }
        println!();
    }
}

fn double_digit(input: &u8) -> u8 {
    let doubled_input = input * 2;
    println!(
        "input to double: {}; after doubling it: {}",
        input, doubled_input
    );
    if doubled_input > 10 {
        1 + doubled_input % 10
    } else {
        doubled_input
    }
}

fn char() {
    let mut sum = 0;
    println!("Enter a six-digit number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let input = input.trim();
    let mut counter = 0;
    for (i, b) in input.as_bytes().iter().enumerate() {
        let b = b - b'0';
        counter += 1;
        println!("position is {counter}; i is {i}; byte is {b}; sum is: {sum}");
        if counter % 2 == 0 {
            sum += b as i32
        } else {
            sum += double_digit(&b) as i32
        }
    }

    println!("sum is: {sum}");
    if sum % 10 == 0 {
        println!("Checksum is divisible by 10. Valid");
    } else {
        println!("Checksum is not divisible by 10. Invalid");
    }
}

enum ModeType {
    UPPERCASE,
    LOWERCASE,
    PUNCTUATION,
}

fn switch_mode() {
    println!("Enter some numbers ending with -1: ");
    let mut input = String::new();
    let mut mode = ModeType::UPPERCASE;

    let mut number: i32;

    loop {
        println!("Number read: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        let mut input: i32 = input.trim().parse().expect("Error passing input");
        println!("input: {input}");
        if input == -1 {
            break;
        }

        match mode {
            ModeType::UPPERCASE => {
                number = input % 27;
                println!("Modulo 27: {number}");
                if number == 0 {
                    println!("Switch to LOWERCASE");
                    mode = ModeType::LOWERCASE;
                }
            }
            ModeType::LOWERCASE => {
                number = input % 27;
                println!("Modulo 27: {number}");
                if number == 0 {
                    println!("Switch to PUNCTUATION");
                    mode = ModeType::PUNCTUATION;
                }
            }
            ModeType::PUNCTUATION => {
                number = input % 9;
                println!("Modulo 9: {number}");
                if number == 0 {
                    println!("Switch to UPPERCASE");
                    mode = ModeType::UPPERCASE;
                }
            }
        }
    }
}

fn match_test() {

    let number = 13;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}

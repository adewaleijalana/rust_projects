pub mod color;

use std::io;

fn main() {
    println!("Welcome to Temperature Converter Utility!");

    loop {
        println!("Enter the temperature value: ");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Error reading user input");

        let temp: f64 = temp.trim().parse().expect("Error getting input");

        println!("Enter the temperature unit to convert to: C for Celsius to Fahrenheit | F for Fahrenheit to Celsius");

        let mut temp_unit = String::new();

        io::stdin()
            .read_line(&mut temp_unit)
            .expect("Error reading user input");

        let temp_unit = temp_unit.trim();

        let result = if temp_unit.eq_ignore_ascii_case("C") {
            convert_celcius_to_fahrenheit(&temp)
        } else {
            convert_fahrenheit_to_celcius(&temp)
        };

        if temp_unit.eq_ignore_ascii_case("C") {
            println!("Temperature value: {temp}C Result: {result}F");
        } else {
            println!("Temperature value: {temp}F Result: {result}C");
        }

        println!("Do you want to perform another convertion? Y/N");

        let mut decision = String::new();

        io::stdin()
            .read_line(&mut decision)
            .expect("Error reading user input");

        if decision.trim().eq_ignore_ascii_case("N") {
            break;
        }
    }
}

fn convert_fahrenheit_to_celcius(f: &f64) -> f64 {
    (f - 32_f64) * 5_f64 / 9_f64
}

fn convert_celcius_to_fahrenheit(c: &f64) -> f64 {
    (c * 9_f64 / 5_f64) + 32_f64
}

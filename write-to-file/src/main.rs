use std::{
    fs::{self},
    io::Error,
};
fn main() {
    match write_to_file() {
        Ok(_) => println!("File written successfully."),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}

fn write_to_file() -> Result<(), Error> {

    let hotel = Hotel;
    let airbnb = AirBnB;

    let stays: Vec<&dyn Accommodation> = vec![&hotel, &airbnb];

    for stay in stays {
        stay.description();
    }   

    // let input = std::io::stdin();
    // println!("What file would you like to write to?");
    // let mut file_name = String::new();
    // input.read_line(&mut file_name)?;
    // let file_name = file_name.trim();

    // println!("What would you like to write to the file?");
    // let mut content = String::new();
    // input.read_line(&mut content)?;
    // let content = content.trim();

    // fs::write(file_name, content.as_bytes())?;
    Ok(())
}

trait Accommodation {
    fn description(&self) {
        println!("This is a place to stay.");
    }
}

trait Booking {
    fn booking(&self) {
        println!("This is a booking method.");
    }
}

struct Hotel;

struct AirBnB;

impl Accommodation for Hotel {
    fn description(&self) {
        println!("This is a place to stay.");
    }
}
impl Booking for Hotel {
    fn booking(&self) {
        println!("This is a booking method.");
    }
}

impl Accommodation for AirBnB {
    fn description(&self) {
        println!("This is a place to stay.");
    }
}
impl Booking for AirBnB {
    fn booking(&self) {
        println!("This is a booking method.");
    }
}

fn show_accommodation_info<>(test: &(impl Accommodation + Booking)) {
    test.description();
    test.booking();
}
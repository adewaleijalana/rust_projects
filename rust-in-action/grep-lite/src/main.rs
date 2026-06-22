#![allow(dead_code, unused_variables, unused_imports)]

use grep_lite::file::{File, close, open};
use grep_lite::read::Read;
use grep_lite::reader_args;

fn main() {
    // reader_args()

    let mut f3 = File::new("2.txt");

    let mut buffer: Vec<u8> = vec![];

    if f3.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f3 = open(f3).unwrap();
    let f2_length = f3.read(&mut buffer).unwrap();
    f3 = close(f3).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f3);
    println!("{}", f3);
    println!("{} is {} bytes long", &f3.get_name(), f2_length);
    println!("{}", text)
}

#![allow(dead_code, unused_variables, unused_imports)]

use grep_lite::file::{File, close, open, read};
use grep_lite::reader_args;

fn main() {
    // reader_args()

    let mut f1 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f1);
    let f2_length = read(&f1, &mut buffer);
    close(&mut f1);

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, f2_length);
    println!("{}", text)
}

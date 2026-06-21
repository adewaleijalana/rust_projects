#![allow(dead_code, unused_variables, unused_imports)]

use grep_lite::file::{File, close, open};
use grep_lite::reader_args;

fn main() {
    // reader_args()

    let f3_data = vec![114, 117, 115, 116, 33];

    let mut f1 = File::new_with_data("2.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    open(&mut f1);
    let f2_length = f1.read(&mut buffer);
    close(&mut f1);

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.get_name(), f2_length);
    println!("{}", text)
}

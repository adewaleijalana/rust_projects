use std::{env, fs::File, io::prelude::*};

const BYTES_PER_LINE: usize = 16;

const INPUT: &'static [u8] = br#"
 fn main() {
 println!("Hello, world!");
 }"#;

pub fn hexdump_example() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    // INPUT.read_to_end(&mut buffer)?;

    buffer.extend_from_slice(INPUT);

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}

pub fn file_hexdump() {
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: fview FILENAME");
    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];
    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }
}
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let result = read_file();
    match result {
        Ok(()) => println!("done"),
        Err(err) => println!("here is the error {:?}", err)
    };
}

#[allow(unused_variables)]
fn read_file() -> std::io::Result<()> {
    println!("opening a file");
    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file).lines();
    for line in  reader.flatten(){
        let ln: Vec<&str> = line.split(", ").collect();
        println!("{:?}", ln);
        for l in  &ln {
            println!("each word after split: {}", l);
        }
    }
  Ok(())
}

#![allow(dead_code, unused_variables, unused_imports)]

use clap::{Arg, command};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub mod file;
pub mod read;
pub mod sat;

pub fn process_lines<T: BufRead + Sized>(reader: T, search_reg: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let contain_substring = search_reg.find(&line);

        if contain_substring.is_some() {
            println!();
            println!();
            println!("{}", line);
        }
    }
}

pub fn reader_args() {
    let args = command!()
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true),
        )
        .arg(Arg::new("input").help("File to search").required(true))
        .get_matches();

    let pattern = args
        .get_one::<String>("pattern")
        .expect("`pattern`is required");

    let binding = "-".to_string();
    let input = args.get_one::<String>("input").unwrap_or(&binding);

    let search_reg = Regex::new(pattern).unwrap();
    //     let quote = "Every face, every shop, bedroom window, public-house, and
    // dark square is a picture feverishly turned--in search of what?
    // It is the same with books. What do we seek through millions of pages?";

    if *input == binding {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, search_reg);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, search_reg);
    }
}

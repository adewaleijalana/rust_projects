use clap::{Arg, command};
use regex::Regex;

fn main() {
    let matches = command!()
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true),
        )
        .get_matches();

    let search_reg = Regex::new("picture").unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contain_substring = search_reg.find(line);

        match contain_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

use clap::{Arg, ArgAction, Command, arg};
use clock::clock::Clock;

fn main() {
    let matches = Command::new("clock")
        .version("1.0")
        .about("Gets and (aspirationally) sets the time.")
        .arg(
            Arg::new("action")
                .action(ArgAction::Set)
                .default_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::new("std")
                .short('s')
                .long("use-standard")
                .action(ArgAction::Set)
                .default_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(
            Arg::new("datetime")
                .help("When <action> is 'set', apply <datetime>. Otherwise, ignore."),
        )
        .get_matches();

    let action = matches.get_one::<String>("action").unwrap();

    let std = matches.get_one::<String>("std").unwrap();

    if action == "set" {
        unimplemented!()
    }

    let now = Clock::get();

    match std.as_str() {
        "timestamp" => println!("timestamp: {}", now.timestamp()),
        "rfc2822" => println!("rfc2822: {}", now.to_rfc2822()),
        "rfc3339" => println!("rfc3339: {}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}

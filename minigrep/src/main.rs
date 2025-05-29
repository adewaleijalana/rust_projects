use std::env;
use std::process;
use std::process::Command;

use minigrep::config::Config;
use minigrep::run;
fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // run_cmd();
}

fn run_cmd() {
    let output = Command::new("echo")
        .arg("Hello from Rust!")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output: {}", stdout.trim());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr.trim());
    }
}

#![allow(dead_code, unused_variables)]

use std::io::Write;
use std::{env, io};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::sync::mpsc::{Sender, channel};
mod args;

use args::Args;

const MAX_PORT: u16 = 65535;

fn main() {
    println!("Welcome to the Port Sniffer CLI program!");
    let args: Vec<String> = env::args().collect();

    let args = Args::new(&args).unwrap_or_else(|e| {
        if e.contains("help") {
            process::exit(0);
        } else {
            eprintln!("Problem parsing arguments: {}", e);
            process::exit(1);
        }
    });

    println!("{args:#?}");

    let threads = args.threads();
    let ip = args.ip();

    let (tx, rx) = channel();

    for i in 0..threads {
        let tx = tx.clone();

        std::thread::spawn(move || {
            scan_ports(tx, i, ip, threads);

        });

        
    }

    let mut out = Vec::new();
    drop(tx);

    for r in rx {
        out.push(r);
    }

    println!(" ");

    for o in out {
        println!("Port {o} is open")
    }

}

fn scan_ports(tx: Sender<u16>, start_port: u16, ip: IpAddr, threads: u16) {
    let mut port = start_port + 1;
    loop {

        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(start_port).unwrap();
            },
            Err(_) => {

            }
        }

        if MAX_PORT - port <= threads {
            break;
        }

        port += threads;
    }
}

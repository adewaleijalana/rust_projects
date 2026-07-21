use std::thread;

use crossbeam::{channel::unbounded, select};

pub fn run_simple_channel() {
    let (tx, rx) = unbounded::<i32>();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    select! {
        recv(rx) -> msg => println!("{:?}", msg),
    }
}

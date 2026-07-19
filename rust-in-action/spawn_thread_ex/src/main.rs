use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let start = Instant::now();

    let handler = thread::spawn(|| {
        println!("----- waiting -----");
        let pause = Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    handler.join().unwrap();

    let finish = Instant::now();

    println!("{:02?}", finish.duration_since(start));
}

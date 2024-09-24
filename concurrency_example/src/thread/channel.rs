use std::{sync::mpsc, thread};

pub fn channel_example() {
  let (sender, receiver) = mpsc::channel();

  thread::spawn(move || {
    let mssg = String::from("Hello, Rose");
    sender.send(mssg).unwrap();
  });
    
    let received = receiver.recv().unwrap();
    println!("Received this message: {received}")
}
use std::thread;


fn main() {
    // borrows_mutably_ex();
    closure_move_ex();
}

#[allow(dead_code)]
fn borrows_mutably_ex() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || {
        list.push(7);
    };

    borrows_mutably();
    println!("After calling closure: {list:?}");
}

#[allow(dead_code)]
fn closure_move_ex() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

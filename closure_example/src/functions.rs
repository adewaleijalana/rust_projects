use std::thread;

pub fn borrows_mutably_ex() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = |value: i32| {
        list.push(value);
    };

    borrows_mutably(7);
    borrows_mutably(17);
    borrows_mutably(27);
    println!("After calling closure: {list:?}");
}

pub fn closure_move_ex() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

pub fn borrows_immutably_ex() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let borrows_immutably = || {
        println!("Inside a closure{:?}", list)
    };

    borrows_immutably();
    borrows_immutably();
    borrows_immutably();
    println!("After calling closure: {list:?}");
}
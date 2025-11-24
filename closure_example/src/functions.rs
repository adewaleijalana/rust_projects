use std::{io::ErrorKind, thread};

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

    let borrows_immutably = || println!("Inside a closure{:?}", list);

    borrows_immutably();
    borrows_immutably();
    borrows_immutably();
    println!("After calling closure: {list:?}");
}

pub fn unwrap_or_else_test_option() {
    let option = Some("Salamin");
    let food = option.unwrap_or_else(|| "Pizza");
    println!("Food from Some(x): {food}");

    let option = None;
    let food = option.unwrap_or_else(|| "Pizza");
    println!("Food from None: {food}");
}

pub fn unwrap_or_else_test_result() {
    let result = Ok("Salamin");
    let food = result.unwrap_or_else(|_: &str| "Pizza");
    println!("Food from Ok(x): {food}");

    let result = Err("test");
    let food = result.unwrap_or_else(|e| {
        if e.eq_ignore_ascii_case("Test") {
            "Pizza"
        } else {
            "Bugger"
        }
    });

    // let result = Err(ErrorKind::InvalidData);
    // let result = Err(ErrorKind::InvalidInput);
    // let food = result.unwrap_or_else(|e| match e {
    //     ErrorKind::InvalidData => "Pizza",
    //     _ => "Bugger",
    // });

    println!("Food from Err(e): {food}");
}

pub fn closure_function<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn test<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

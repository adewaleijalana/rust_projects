use std::ops::Deref;

use bills_pay_service;
fn main() {
    println!("Hello, Workspace from the main method!");
    bills_pay_service::service_test();

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T>  MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
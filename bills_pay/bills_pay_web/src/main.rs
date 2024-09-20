use std::ops::Deref;

use bills_pay_service;
fn main() {
    // println!("Hello, Workspace from the main method!");
    // bills_pay_service::service_test();

    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    let nums = [22, 3, 45, 5, 65];

    let result = largest_number(&nums);
    println!("largest number is {result}")
}


struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T>  MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn largest_number(nums: &[i32]) -> i32 {
    let mut max = nums[0];

    // let res = nums.iter().max().unwrap();
    // println!("{res}");

    for num in nums.iter() {
        if *num > max {
            max = *num;
        }
    }
    max
}
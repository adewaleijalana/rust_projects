#![allow(dead_code)]
fn main() {
	println!("{}", add(2, 5));
	// println!("{}", add2(22, 5));
}

fn add(a: i32, b: i32) -> i32{
	a + b
}

fn add2(a: i32, b: i32){
	let _ = a + b;
}
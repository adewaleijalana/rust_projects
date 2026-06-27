use std::{borrow::Cow, ffi::CStr, os::raw::c_char};

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn print_addr() {
    let a = 42;
    let b = &B;
    let c = &C;

    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}

pub fn print_addr_2() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;

    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!(" location: {:p}", &a);
    println!(" size: {:?} bytes", size_of::<usize>());
    println!(" value: {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!(" location: {:p}", &b);
    println!(" size: {:?} bytes", size_of::<&[u8; 10]>());
    println!(" points to: {:p}", b);
    println!();

    println!("c (a \"box\" for C):");
    println!(" location: {:p}", &c);
    println!(" size: {:?} bytes", size_of::<Box<[u8]>>());
    println!(" points to: {:p}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!(" location: {:p}", &B);
    println!(" size: {:?} bytes", size_of::<[u8; 10]>());
    println!(" value: {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!(" location: {:p}", &C);
    println!(" size: {:?} bytes", size_of::<[u8; 11]>());
    println!(" value: {:?}", C);
}

pub fn print_addr_3() {
    let a = 42;
    let b: String;

    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
}

pub fn int_vs_int() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}

pub fn transmute_test() {
    let a: f32 = 42.42;
    // let frankentype: u32 = unsafe { std::mem::transmute(a) }; 

    let frankentype: u32 = f32::to_bits(a); 

    //f32::to_bits(a)

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    // let b: f32 = unsafe { std::mem::transmute(frankentype) };
     let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("{}", b);
    assert_eq!(a, b);
}

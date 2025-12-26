pub fn test_fn() {
    let mut sushi = String::from("Yellowtail");
    let sushi_ref = &mut sushi;
    // let sushi_ref2 = &mut sushi; //will not compile rustc --explain E0502

    // println!("{sushi_ref} and {sushi_ref2}");
}

pub fn raw_pointer_test() {
    let mut sushi = String::from("Yellowtail");
    let sushi_ref = &raw const sushi;
    let sushi_ref2: *const String = &sushi;
    let sushi_ref_mutable_pointer_1 = &raw mut sushi;
    let sushi_ref_mutable_pointer_2 = &raw mut sushi;

    drop(sushi);

    unsafe {
        println!("{}", *sushi_ref);
    }
}

pub fn smart_pointer() {
    let my_box = Box::new(100);
    println!("{:#?}", my_box)
}

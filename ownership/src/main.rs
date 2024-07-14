// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }


fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("in main {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}
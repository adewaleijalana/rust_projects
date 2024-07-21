mod lifetime;

#[allow(unused_imports)]
use lifetime::{longest, test_life_time};

fn main() {
    //    test_life_time::lifetime_test_error();
    //    test_life_time::lifetime_test_2()

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest::longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

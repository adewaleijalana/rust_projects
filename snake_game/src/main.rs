mod window;

use std::collections::HashMap;

use window::frame::create_window;

fn main() {
    create_window();
}

// fn main() {
//     let mut likes: HashMap<&str, String> = HashMap::new();

//     likes.entry("Nouman").or_insert("apple".to_string());
//     likes
//         .entry("Nouman")
//         .or_insert("mango".to_string())
//         .push_str("banana");
//     println!("{:?}", likes.get("Nouman").unwrap());
// }

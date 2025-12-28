#![allow(unused, dead_code)]

use rust_pointers_examples::clothings::tie::Tie;
use rust_pointers_examples::clothings::wearable::Wearable;
use rust_pointers_examples::linked_list::LinkedList;
use rust_pointers_examples::{clothings::pants::Pants, functions::*};
use std::ops::Deref;

use rust_pointers_examples::{
    binary_search_tree::BinarySearchTree, music_playlist_item::MusicPlaylistItem,
};
fn main() {
    // raw_pointer_test();
    // smart_pointer();

    // let mut binary_search_tree = BinarySearchTree::new();

    // binary_search_tree.insert(5);

    // binary_search_tree.insert(2);

    // binary_search_tree.insert(8);

    // binary_search_tree.insert(7);

    // binary_search_tree.insert(1);

    // println!("{:#?}", binary_search_tree);

    // println!("{}", binary_search_tree.contains(5));

    // println!("{}", binary_search_tree.contains(15))

    // let arr = vec![1, 2, 4, 5];
    // let arr_deff = arr.deref();
    // print_value(&arr);
    // println!();
    // print_value(arr_deff);

    // let pant = Pants::new("Cotton".to_string(), 32);
    // let tie = Tie::new("Blue".to_string());
    // let wearables: Vec<&dyn Wearable> = vec![&pant, &tie];

    // wearables_ref(wearables);

    match read_number_from_file_4("value.txt"){
        Ok(value) => println!("The value is {}", value),
        Err(error) => println!("The error is: {error}")
        
    }
}

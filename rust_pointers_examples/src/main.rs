#![allow(unused, dead_code)]

use rust_pointers_examples::functions::*;
use rust_pointers_examples::linked_list::LinkedList;

use rust_pointers_examples::{binary_search_tree::BinarySearchTree, music_playlist_item::MusicPlaylistItem};
fn main() {
    // raw_pointer_test();
    // smart_pointer();

    let mut binary_search_tree = BinarySearchTree::new();

    binary_search_tree.insert(5);

    binary_search_tree.insert(2);

    binary_search_tree.insert(8);

    binary_search_tree.insert(7);

    binary_search_tree.insert(1);

    println!("{:#?}", binary_search_tree);

    println!("{}", binary_search_tree.contains(5));

    println!("{}", binary_search_tree.contains(15))
}

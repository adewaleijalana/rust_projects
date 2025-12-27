#![allow(unused, dead_code)]
mod binary_search_tree;
mod functions;
mod linked_list;
mod music_playlist_item;

use functions::*;
use linked_list::LinkedList;

use crate::{binary_search_tree::BinarySearchTree, music_playlist_item::MusicPlaylistItem};
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

    println!("{}", binary_search_tree.contains(5))
}

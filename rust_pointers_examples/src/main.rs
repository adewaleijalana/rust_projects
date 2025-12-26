#![allow(unused, dead_code)]
mod functions;
mod linked_list;
mod music_playlist_item;

use functions::*;
use linked_list::LinkedList;
fn main() {
    // raw_pointer_test();
    // smart_pointer();

    let list_1 = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Empty),
    };

    println!("{:?}", list_1);

    let list_2 = LinkedList::Node {
        value: 2,
        next: Box::new(list_1),
    };

    println!("{:?}", list_2);

    let list_3 = LinkedList::Node {
        value: 3,
        next: Box::new(list_2),
    };

    println!("{:?}", list_3);
}

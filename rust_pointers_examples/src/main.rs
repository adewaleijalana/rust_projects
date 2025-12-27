#![allow(unused, dead_code)]
mod functions;
mod linked_list;
mod music_playlist_item;

use functions::*;
use linked_list::LinkedList;

use crate::music_playlist_item::MusicPlaylistItem;
fn main() {
    // raw_pointer_test();
    // smart_pointer();

    let list_1 = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Node {
                value: 3,
                next: Box::new(LinkedList::Empty),
            }),
        }),
    };

    // println!("{:?}", list_1);

    let music_playlist_item_3 = MusicPlaylistItem {
        artist: String::from("Artist 3"),
        name: String::from("Song 3"),
        next_track: None,
    };

    // println!("next track: {:#?}", music_playlist_item_3.next_track);
    // println!("");

    let music_playlist_item_2 = MusicPlaylistItem {
        artist: String::from("Artist 2"),
        name: String::from("Song 2"),
        next_track: Some(&music_playlist_item_3),
    };

    // println!("next track: {:#?}", music_playlist_item_2.next_track);
    // println!("");

    let music_playlist_item_1 = MusicPlaylistItem {
        artist: String::from("Artist 1"),
        name: String::from("Song 1"),
        next_track: Some(&music_playlist_item_2),
    };

    println!("{:#?}", music_playlist_item_1);

    let mut next_music_item = music_playlist_item_1.next_track;

    while let Some(music_item) = next_music_item {
        println!("next track: {:#?}", music_item);
        next_music_item = music_item.next_track
    }
}

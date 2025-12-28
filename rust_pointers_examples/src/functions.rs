use std::{
    error::Error,
    fmt::Display,
    fs,
    io::{self, Error as IoError},
};

use crate::{
    clothings::{
        pants::Pants,
        tie::{self, Tie},
        wearable::{self, Wearable},
    },
    linked_list::LinkedList,
    music_playlist_item::MusicPlaylistItem,
};

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

pub fn linked_list_test() {
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

    println!("{:?}", list_1);
}

pub fn music_playlist_method() {
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

pub fn print_value<T: Display>(arr: &[T]) {
    for t in arr {
        println!("{}", t)
    }
}

pub fn wearables() -> Vec<Box<dyn Wearable>> {
    let pant = Pants::new("Cotton".to_string(), 32);
    let tie = Tie::new("Blue".to_string());
    vec![Box::new(pant), Box::new(tie)]
}

pub fn wearables_ref(wearables: Vec<&dyn Wearable>) {
    for wearable in wearables {
        println!("{}", wearable.wear());
    }
}

pub fn read_number_from_file(path: &str) -> Result<i32, io::Error> {
    let file_content = fs::read_to_string(path)?;
    let parse_number = file_content
        .parse::<i32>()
        .map_err(|e| IoError::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(parse_number)
}

pub fn read_number_from_file_2(path: &str) -> Result<i32, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let parse_number = file_content.parse::<i32>()?;

    Ok(parse_number)
}


pub fn read_number_from_file_4(path: &str) -> Result<i32, Box<dyn Error>> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => return Err(Box::new(error))
    };
    
    let parse_number = match file_content.parse::<i32>(){
        Ok(num) => num,
        Err(error) => return Err(Box::new(error))
    };

    Ok(parse_number)
}

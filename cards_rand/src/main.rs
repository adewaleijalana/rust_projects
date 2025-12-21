#![allow(unused)]
use crate::card_deck::deck::Deck;

mod card_deck;

fn main() {
    let mut deck = Deck::new();

    // println!("deck before shuffle: {:#?}", deck);

    deck.shuffle();

    // println!("deck after shuffle: {:#?}", deck);

    // deck.insert_jokers();

    // println!("deck after inserting jokers: {:#?}", deck);

    for i in (1..=10) {
        deck.delete_random_card();
        println!("size: {} at iteration: {}", deck.card_size(), i)
    }
}

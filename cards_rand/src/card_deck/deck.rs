use rand::{rng, seq::{IndexedRandom, SliceRandom}};

use crate::card_deck::{card::Card, rank::{self, Rank}, suit::Suit};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let suits = Suit::suits();
        let ranks = Rank::ranks();
        

        let cards = (1..=52).map(|_| {
            let mut rng = rng();
            let suit = suits.choose(&mut rng).cloned();
            let rank = ranks.choose(&mut rng).unwrap().clone();
            Card::new(rank, suit)
        }).collect::<Vec<Card>>();

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
}

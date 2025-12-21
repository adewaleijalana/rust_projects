use rand::{Rng, rng, seq::{IndexedRandom, SliceRandom}};

use crate::card_deck::{card::Card, rank::{self, Rank}, suit::Suit};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let suits = Suit::suits();
        let ranks = Rank::ranks();

        let mut cards = Vec::new();
        

        // let cards = (1..=52).map(|_| {
        //     let mut rng = rng();
        //     let suit = suits.choose(&mut rng).cloned();
        //     let rank = ranks.choose(&mut rng).unwrap().clone();
        //     Card::new(rank, suit)
        // }).collect::<Vec<Card>>();

        for suit in suits.into_iter() {
            for rank in ranks.clone().into_iter() {
                cards.push(Card::new(rank, Some(suit)));
            }
        }

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn insert_jokers(&mut self) {
        let mut my_rang = rng();

        for _ in 0..2 {
            let randon_index = my_rang.random_range(0..self.cards.len());
            let joker_card = Card::new(Rank::Joker, None);
            self.cards.insert(randon_index, joker_card);
        }
    }

    pub fn delete_random_card(&mut self) {
        self.cards.pop_if(|card| {
            rand::random_bool(13.0 / 20.0)
        });
    }

    pub fn card_size(&self) -> usize {
        self.cards.len()
    }
}

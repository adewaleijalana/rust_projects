use crate::card_deck::{rank::Rank, suit::Suit};

#[derive(Debug)]
pub struct Card {
    rank: Rank,
    suit: Option<Suit>,
}

impl Card {
    pub fn new(rank: Rank, suit: Option<Suit>) -> Self {
        Self { rank, suit }
    }
}

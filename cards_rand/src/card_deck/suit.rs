#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

impl Suit {
    pub fn suits() -> Vec<Self> {
        vec![Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds]
    }
}

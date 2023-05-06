#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    pub fn all() -> Vec<Suit> {
        vec![Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade]
    }
}
use std::fmt;
use std::fmt::Formatter;
use crate::face::Face;
use crate::orientation::Orientation;
use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub face: Face,
    pub orientation: Orientation,
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            face: Face::Up,
            orientation: Orientation::Upright,
            suit: suit.clone(),
            rank: rank.clone(),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}s", self.rank, self.suit)
    }
}
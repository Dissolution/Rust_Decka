use std::fmt::*;
use crate::misc::LongDisplay;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade]
    }
}

impl LongDisplay for Suit {
    fn display(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Suit::Club => {
                write!(f, "C")
            }
            Suit::Diamond => {
                write!(f, "D")
            }
            Suit::Heart => {
                write!(f, "H")
            }
            Suit::Spade => {
                write!(f, "S")
            }
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Suit::Club => {
                write!(f, "♣")
            }
            Suit::Diamond => {
                write!(f, "♦")
            }
            Suit::Heart => {
                write!(f, "♥")
            }
            Suit::Spade => {
                write!(f, "♠")
            }
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,        // Ace is high
}

impl Rank {
    pub fn all() -> [Rank; 13] {
        [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }
}


impl LongDisplay for Rank {
    fn display(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Rank::Ace => {
                write!(f, "A")
            }
            Rank::Two => {
                write!(f, "2")
            }
            Rank::Three => {
                write!(f, "3")
            }
            Rank::Four => {
                write!(f, "4")
            }
            Rank::Five => {
                write!(f, "5")
            }
            Rank::Six => {
                write!(f, "6")
            }
            Rank::Seven => {
                write!(f, "7")
            }
            Rank::Eight => {
                write!(f, "8")
            }
            Rank::Nine => {
                write!(f, "9")
            }
            Rank::Ten => {
                write!(f, "X")
            }
            Rank::Jack => {
                write!(f, "J")
            }
            Rank::Queen => {
                write!(f, "Q")
            }
            Rank::King => {
                write!(f, "K")
            }
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.display(f)
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    //pub face: Face,
    //pub orientation: Orientation,
    pub suit: Suit,
    pub rank: Rank,
}

impl LongDisplay for Card {
    fn display(&self, f: &mut Formatter<'_>) -> Result {
        //f.write_char('[')?;
        self.rank.display(f)?;
        self.suit.display(f)?;
        //f.write_char(']')?;
        Ok(())
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} of {:?}s", self.rank, self.suit)
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            //face: Face::Up,
            //orientation: Orientation::Upright,
            suit,
            rank,
        }
    }
}


// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Orientation {
//     Upright,
//     Inverted,
// }
//
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Face {
//     Up,
//     Down,
// }
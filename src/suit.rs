use crate::format::Formattable;
use std::fmt::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
//#[non_exhaustive]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

impl Suit {
    pub const MEMBERS: [Suit; 4] = [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade];

    #[allow(dead_code)]
    pub fn is_red(&self) -> bool {
        match self {
            Suit::Diamond | Suit::Heart => true,
            Suit::Club | Suit::Spade => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_black(&self) -> bool {
        match self {
            Suit::Diamond | Suit::Heart => false,
            Suit::Club | Suit::Spade => true,
        }
    }
}

impl TryFrom<u8> for Suit {
    type Error = &'static str;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        // can fit into 2 bits
        match value {
            0b00 => Ok(Suit::Diamond),
            0b01 => Ok(Suit::Club),
            0b10 => Ok(Suit::Heart),
            0b11 => Ok(Suit::Spade),
            _ => Err("Suit only uses the first two bits"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

impl Formattable for Suit {
    fn format_emoji(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Suit::Club => {
                write!(f, "♣️")
            }
            Suit::Diamond => {
                write!(f, "♦️")
            }
            Suit::Heart => {
                write!(f, "♥️")
            }
            Suit::Spade => {
                write!(f, "♠️")
            }
        }
    }

    fn format_short(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Suit::Club => {
                write!(f, "♣") // ♧
            }
            Suit::Diamond => {
                write!(f, "♦") // ♢
            }
            Suit::Heart => {
                write!(f, "♥") // ♡
            }
            Suit::Spade => {
                write!(f, "♠") // ♤
            }
        }
    }

    fn format_long(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

#[allow(dead_code)]
pub struct SuitOrder;

impl SuitOrder {
    /// https://en.wikipedia.org/wiki/High_card_by_suit
    #[allow(dead_code)]
    pub const ALPHABETICAL: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
    #[allow(dead_code)]
    pub const ALTERNATING_COLORS: [Suit; 4] = [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade];
    #[allow(dead_code)]
    pub const RUSSIAN_AUSTRALIAN: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Diamond, Suit::Heart];
    #[allow(dead_code)]
    pub const GERMAN: [Suit; 4] = [Suit::Diamond, Suit::Heart, Suit::Spade, Suit::Club];
}

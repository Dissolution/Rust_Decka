
use std::fmt::*;
use crate::format::Formattable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
    Ace,
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
}

impl Rank {
    pub const MEMBERS: [Rank; 13] = [
        Rank::Ace,
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
    ];
}

impl TryFrom<u8> for Rank {
    type Error = &'static str;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        // can fit into 4 bits
        match value {
            //0b0000 => Err("0b0000 is the default value"),
            0b0001 => Ok(Rank::Ace),
            0b0010 => Ok(Rank::Two),
            0b0011 => Ok(Rank::Three),
            0b0100 => Ok(Rank::Four),
            0b0101 => Ok(Rank::Five),
            0b0110 => Ok(Rank::Six),
            0b0111 => Ok(Rank::Seven),
            0b1000 => Ok(Rank::Eight),
            0b1001 => Ok(Rank::Nine),
            0b1010 => Ok(Rank::Ten),
            0b1011 => Ok(Rank::Jack),
            0b1100 => Ok(Rank::Queen),
            0b1101 => Ok(Rank::King),
            _ => Err("Rank ranges from 0b0001 to 0b1101"),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

impl Formattable for Rank {
    fn format_emoji(&self, f: &mut Formatter<'_>) -> Result {
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

    fn format_short(&self, f: &mut Formatter<'_>) -> Result {
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

    fn format_long(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

pub struct RankOrder;

impl RankOrder {
    #[allow(dead_code)]
    pub const ACE_LOW: [Rank; 13] = [
        Rank::Ace,
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
    ];

    #[allow(dead_code)]
    pub const ACE_HIGH: [Rank; 13] = [
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
    ];
}

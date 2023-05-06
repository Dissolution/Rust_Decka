use crate::misc::LongDisplay;
use crate::rank::Rank;
use crate::suit::Suit;
use std::fmt::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}

impl TryFrom<u8> for Card {
    type Error = &'static str;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        let rank_bits = value & 0b00001111;
        let rank = Rank::try_from(rank_bits)?;

        let suit_bits = (value & 0b00110000) >> 4;
        let suit = Suit::try_from(suit_bits)?;

        let other_bits = (value & 0b11000000) >> 6;
        if other_bits != 0 {
            Err("A card only uses 6 bits")
        } else {
            Ok(Card::new(suit, rank))
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} of {:?}s", self.rank, self.suit)
    }
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

use crate::format::Formattable;
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

impl Formattable for Card {
    fn format_emoji(&self, f: &mut Formatter<'_>) -> Result {
        match (self.rank, self.suit) {
            (Rank::Ace, Suit::Spade) => f.write_str("🂡"),
            (Rank::Two, Suit::Spade) => f.write_str("🂢"),
            (Rank::Three, Suit::Spade) => f.write_str("🂣"),
            (Rank::Four, Suit::Spade) => f.write_str("🂤"),
            (Rank::Five, Suit::Spade) => f.write_str("🂥"),
            (Rank::Six, Suit::Spade) => f.write_str("🂦"),
            (Rank::Seven, Suit::Spade) => f.write_str("🂧"),
            (Rank::Eight, Suit::Spade) => f.write_str("🂨"),
            (Rank::Nine, Suit::Spade) => f.write_str("🂩"),
            (Rank::Ten, Suit::Spade) => f.write_str("🂪"),
            (Rank::Jack, Suit::Spade) => f.write_str("🂫"),
            //(Rank::Knight, Suit::Spade) => f.write_str("🂬"),
            (Rank::Queen, Suit::Spade) => f.write_str("🂭"),
            (Rank::King, Suit::Spade) => f.write_str("🂮"),
            //(Rank::Joker, Suit::Spade) => f.write_str("🃏"),
            (Rank::Ace, Suit::Heart) => f.write_str("🂱"),
            (Rank::Two, Suit::Heart) => f.write_str("🂲"),
            (Rank::Three, Suit::Heart) => f.write_str("🂳"),
            (Rank::Four, Suit::Heart) => f.write_str("🂴"),
            (Rank::Five, Suit::Heart) => f.write_str("🂵"),
            (Rank::Six, Suit::Heart) => f.write_str("🂶"),
            (Rank::Seven, Suit::Heart) => f.write_str("🂷"),
            (Rank::Eight, Suit::Heart) => f.write_str("🂸"),
            (Rank::Nine, Suit::Heart) => f.write_str("🂹"),
            (Rank::Ten, Suit::Heart) => f.write_str("🂺"),
            (Rank::Jack, Suit::Heart) => f.write_str("🂻"),
            //(Rank::Knight, Suit::Heart) => f.write_str("🂼"),
            (Rank::Queen, Suit::Heart) => f.write_str("🂽"),
            (Rank::King, Suit::Heart) => f.write_str("🂾"),
            //(Rank::Joker, Suit::Heart) => f.write_str("🂿"),
            (Rank::Ace, Suit::Diamond) => f.write_str("🃁"),
            (Rank::Two, Suit::Diamond) => f.write_str("🃂"),
            (Rank::Three, Suit::Diamond) => f.write_str("🃃"),
            (Rank::Four, Suit::Diamond) => f.write_str("🃄"),
            (Rank::Five, Suit::Diamond) => f.write_str("🃅"),
            (Rank::Six, Suit::Diamond) => f.write_str("🃆"),
            (Rank::Seven, Suit::Diamond) => f.write_str("🃇"),
            (Rank::Eight, Suit::Diamond) => f.write_str("🃈"),
            (Rank::Nine, Suit::Diamond) => f.write_str("🃉"),
            (Rank::Ten, Suit::Diamond) => f.write_str("🃊"),
            (Rank::Jack, Suit::Diamond) => f.write_str("🃋"),
            //(Rank::Knight, Suit::Diamond) => f.write_str("🃌"),
            (Rank::Queen, Suit::Diamond) => f.write_str("🃍"),
            (Rank::King, Suit::Diamond) => f.write_str("🃎"),
            //(Rank::Joker, Suit::Diamond) => f.write_str("🂿"),
            (Rank::Ace, Suit::Club) => f.write_str("🃑"),
            (Rank::Two, Suit::Club) => f.write_str("🃒"),
            (Rank::Three, Suit::Club) => f.write_str("🃓"),
            (Rank::Four, Suit::Club) => f.write_str("🃔"),
            (Rank::Five, Suit::Club) => f.write_str("🃕"),
            (Rank::Six, Suit::Club) => f.write_str("🃖"),
            (Rank::Seven, Suit::Club) => f.write_str("🃗"),
            (Rank::Eight, Suit::Club) => f.write_str("🃘"),
            (Rank::Nine, Suit::Club) => f.write_str("🃙"),
            (Rank::Ten, Suit::Club) => f.write_str("🃚"),
            (Rank::Jack, Suit::Club) => f.write_str("🃛"),
            //(Rank::Knight, Suit::Club) => f.write_str("🃜"),
            (Rank::Queen, Suit::Club) => f.write_str("🃝"),
            (Rank::King, Suit::Club) => f.write_str("🃞"),
            //(Rank::Joker, Suit::Club) => f.write_str("🃏"),
            //_ => Err(fmt::Error::default()),
        }
    }

    fn format_short(&self, f: &mut Formatter<'_>) -> Result {
        self.rank.format_short(f)?;
        self.suit.format_short(f)?;
        Ok(())
    }

    fn format_long(&self, f: &mut Formatter<'_>) -> Result {
        self.rank.format_long(f)?;
        f.write_str(" of ")?;
        self.suit.format_long(f)?;
        f.write_char('s')?;
        Ok(())
    }
}

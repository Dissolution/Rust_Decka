use std::fmt;
use std::fmt::*;

use crate::cards::*;
use crate::misc::LongDisplay;


pub struct Deck {

}

impl Deck {
    pub fn new_standard_52() -> Vec<Card> {
        let mut deck: Vec<Card> = Vec::new(); //::with_capacity(52);
        for rank in Rank::all() {
            for suit in Suit::all() {
                let card = Card::new(suit, rank);
                deck.push(card);
            }
        }
        assert_eq!(deck.len(), 52);
        deck
    }
    pub fn copy_new_standard_52(deck: &mut Vec<Card>) {
        for rank in Rank::all() {
            for suit in Suit::all() {
                let card = Card::new(suit, rank);
                deck.push(card);
            }
        }
        assert_eq!(deck.len(), 52);
    }
}

pub type CardPile = Vec<Card>;

impl LongDisplay for CardPile {
    fn display(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('[')?;
        for card in self.iter() {
            card.display(f)?;
            f.write_char(',')?;
        }
        f.write_char(']')?;
        Ok(())
    }
}


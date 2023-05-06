use std::fmt;
use std::fmt::Formatter;
use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;

pub struct Deck {

}

impl Deck {
    pub fn new_standard_52() -> Vec<Card> {
        let mut deck: Vec<Card> = Vec::new();
        for rank in Rank::all() {
            for suit in Suit::all() {
                let card = Card::new(suit, rank);
                deck.push(card);
            }
        }
        assert_eq!(deck.len(), 52);
        deck
    }
}

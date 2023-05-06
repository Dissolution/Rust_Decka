use std::fmt;
use std::fmt::*;

use crate::card::*;
use crate::card_pile::CardPile;
use crate::misc::LongDisplay;
use crate::rank::Rank;
use crate::suit::Suit;

pub struct Deck;

impl Deck {
    pub fn new_standard_52() -> CardPile {
        let mut deck: Vec<Card> = Vec::new(); //::with_capacity(52);
        for rank in Rank::MEMBERS {
            for suit in Suit::MEMBERS {
                let card = Card::new(suit, rank);
                deck.push(card);
            }
        }
        assert_eq!(deck.len(), 52);
        deck
    }
    pub fn copy_new_standard_52(deck: &mut CardPile) {
        assert_eq!(deck.len(), 0);
        for rank in Rank::MEMBERS {
            for suit in Suit::MEMBERS {
                let card = Card::new(suit, rank);
                deck.push(card);
            }
        }
        assert_eq!(deck.len(), 52);
    }
}

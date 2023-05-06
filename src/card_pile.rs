use crate::card::Card;
use crate::misc::LongDisplay;
use std::fmt::*;

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

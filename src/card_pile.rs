use crate::card::Card;
use crate::format::Formattable;
use std::fmt::*;

pub type CardPile = Vec<Card>;

impl Formattable for CardPile {
    fn format_emoji(&self, f: &mut Formatter<'_>) -> Result {
        for card in self.iter() {
            card.format_emoji(f)?;
        }
        Ok(())
    }

    fn format_short(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('[')?;
        for (i, card) in self.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            card.format_short(f)?;
        }
        f.write_char(']')?;
        Ok(())
    }

    fn format_long(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('[')?;
        for (i, card) in self.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            card.format_long(f)?;
        }
        f.write_char(']')?;
        Ok(())
    }
}

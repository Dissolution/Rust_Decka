use crate::card::Card;
use crate::misc::LongDisplay;
use std::fmt::*;
use crate::format::Formattable;

pub type CardPile = Vec<Card>;

impl Formattable for CardPile {
    fn format_emoji(&self, f: &mut Formatter<'_>) -> Result {
        todo!()
    }

    fn format_short(&self, f: &mut Formatter<'_>) -> Result {
        todo!()
    }

    fn format_long(&self, f: &mut Formatter<'_>) -> Result {
        todo!()
    }
}

// impl LongDisplay for CardPile {
//     fn display(&self, f: &mut Formatter<'_>) -> Result {
//         f.write_char('[')?;
//         for card in self.iter() {
//             card.display(f)?;
//             f.write_char(',')?;
//         }
//         f.write_char(']')?;
//         Ok(())
//     }
// }

use crate::decision::Decision;
use std::fmt::*;

#[derive(Clone, Debug)]
pub struct GameResult {
    pub win: bool,
    pub pile_count: usize,
    pub initial_state: String,
    pub decision_chance: f64,
    pub decisions: Vec<Decision>,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            r#"
            {} - {} cards in piles
            {:.1} "#,
            if self.win { "Win" } else { "Lose" },
            self.pile_count,
            self.decision_chance * 100.0
        )?;
        f.write_char('[')?;
        for decision in self.decisions.iter() {
            Display::fmt(decision, f)?;
        }
        Ok(())
    }
}

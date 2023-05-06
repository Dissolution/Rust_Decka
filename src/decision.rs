use std::fmt::*;

#[derive(Clone, Copy, Debug)]
pub struct Decision {
    pub deal_count: usize,
    pub fill_pile: usize,
    pub hole_pile: usize,
}

impl Display for Decision {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "D{}: {}->{}",
            self.deal_count, self.fill_pile, self.hole_pile
        )
    }
}

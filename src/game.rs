use std::cmp::Ordering;
use std::fmt::*;
use crate::deck::*;
use crate::cards::*;
use crate::misc;
use crate::misc::*;

fn pile_move_card(source_pile: &mut CardPile, dest_pile: &mut CardPile) {
    let card = source_pile.pop().unwrap();
    dest_pile.push(card);
    //println!("Moved {:?}", card);
}

fn suit_rank_compare(left: &Card, right: &Card) -> i8 {
    if left.suit == right.suit {
        match left.rank.cmp(&right.rank) {
            Ordering::Equal => {
                panic!("Duplicate card!");
            }
            Ordering::Less => {
                return -1;
            }
            Ordering::Greater => {
                return 1;
            }
        }
    }
    0
}


#[derive(Clone, Debug)]
pub struct GameResult {
    pub win: bool,
    pub pile_count: usize,
    pub decision_chance: f64,
    pub decisions: Vec<Decision>,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, r#"
            {} - {} cards in piles
            {:.1} "#,
               if self.win { "Win" } else { "Lose" },
               self.pile_count,
               self.decision_chance * 100.0)?;
        f.write_char('[')?;
        for decision in self.decisions.iter() {
            Display::fmt(decision, f)?;
        }
        Ok(())
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Decision {
    pub deal_count: usize,
    pub fill_pile: usize,
    pub hole_pile: usize,
}

impl Display for Decision {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "D{}: {}->{}", self.deal_count, self.fill_pile, self.hole_pile)
    }
}


#[derive(Clone, Debug)]
pub struct GameState {
    pub deck: CardPile,
    pub piles: [CardPile; 4],
    pub discard: CardPile,

    pub decision_chance: f64,
    pub decisions: Vec<Decision>,
}

impl CardDisplay for GameState {
    fn display(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("Deck:")?;
        self.deck.display(f)?;
        f.write_str("\tPileA:")?;
        self.piles[0].display(f)?;
        f.write_str("\tPileB:")?;
        self.piles[1].display(f)?;
        f.write_str("\tPileC:")?;
        self.piles[2].display(f)?;
        f.write_str("\tPileD:")?;
        self.piles[3].display(f)?;
        f.write_str("\tDiscard:")?;
        self.discard.display(f)?;
        Ok(())
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Deck: {:?}\tPileA: {:?}\tPileB: {:?}\tPileC: {:?}\tPileD: {:?}\tDiscard: {:?}",
               self.deck, self.piles[0], self.piles[1], self.piles[2], self.piles[3], self.discard)
    }
}


impl GameState {
    pub fn new() -> GameState {
        GameState {
            deck: Vec::with_capacity(52),
            piles: [Vec::with_capacity(13), Vec::with_capacity(13), Vec::with_capacity(13), Vec::with_capacity(13)],
            discard: Vec::with_capacity(52),
            decision_chance: 1.0,
            decisions: Vec::new(),
        }
    }

    pub fn current_result(&self) -> Option<GameResult> {
        if !self.deck.is_empty() {
            None
        } else {
            let mut pile_count: usize = 0;
            let mut is_win: bool = true;
            for pile in self.piles.iter() {
                pile_count += pile.len();
                match pile.last() {
                    None => {
                        panic!("NOPE");
                    }
                    Some(card) => {
                        if card.rank != Rank::Ace {
                            is_win = false;
                        }
                    }
                }
            }
            Some(GameResult {
                win: is_win,
                pile_count,
                decision_chance: self.decision_chance,
                decisions: self.decisions.clone(),
            })
        }
    }

    pub fn move_top_card(&mut self, source_pile_index: usize, dest_pile_index: usize) {
        let piles = get_two_mut(&mut self.piles, source_pile_index, dest_pile_index);
        match piles {
            None => {}
            Some((source_pile, dest_pile)) => {
                pile_move_card(source_pile, dest_pile)
            }
        }
    }

    pub fn collide(&mut self) {
        'main: loop {
            // midpoint for split is the exclusive upper bound
            // 0 | 1 | 2 | 3
            for mid in 1..=3 {

                // split here
                let (left_half, right_half) = self.piles.split_at_mut(mid);

                // the left pile is the last pile in the left half
                let left_pile = left_half.last_mut().unwrap();
                let left_card = left_pile.last();
                // Pile has to have a card
                if left_card.is_none() {
                    continue;
                }
                let left_card = left_card.unwrap();

                // scan against everything in the right half
                for right_pile in right_half.iter_mut() {
                    let right_card = right_pile.last_mut();
                    if right_card.is_none() {
                        continue;
                    }
                    let right_card = right_card.unwrap();

                    // collide
                    let c = suit_rank_compare(left_card, right_card);
                    match c.cmp(&0i8) {
                        Ordering::Equal => {
                            // do nothing
                        }
                        Ordering::Less => {
                            pile_move_card(left_pile, &mut self.discard);
                            continue 'main;
                        }
                        Ordering::Greater => {
                            pile_move_card(right_pile, &mut self.discard);
                            continue 'main;
                        }
                    }
                }
            }
            // done colliding
            break;
        }
    }

    pub fn play_to_decision(&mut self) -> Option<Vec<GameState>> {
        loop {
            if self.deck.is_empty() {
                //println!("Deck is empty");
                return None;
            }

            // deal a card to each pile
            for i in 0..4 {
                pile_move_card(&mut self.deck, &mut self.piles[i]);
            }

            // run collisions
            self.collide();

            // check for fillable holes
            let holes = self.find_holes();

            if holes.is_empty() {
                continue;
            }
            if holes.len() == 1 && holes[0].1.len() == 1 {
                // This is an easy move!
                let hole_index = &holes[0].0;
                let fill_index = &holes[0].1[0];

                self.move_top_card(*fill_index, *hole_index);

                // Continue processing _this_ game_state
                continue;
            }

            // Have to split!!
            let filler_count: usize = holes.iter()
                .map(|h| h.1.len())
                .sum();
            let multiplier = 1.0 / filler_count as f64;

            let mut game_states = Vec::with_capacity(4);
            for hole in holes.iter() {
                let (hole_index, file_indices) = hole;
                for fill_index in file_indices.iter() {
                    let decision = Decision {
                        deal_count: 13_usize - (self.deck.len() / 4_usize),
                        fill_pile: *fill_index,
                        hole_pile: *hole_index,
                    };
                    let mut clone: GameState = self.clone();
                    clone.decision_chance *= multiplier;
                    clone.decisions.push(decision);
                    clone.move_top_card(*fill_index, *hole_index);
                    game_states.push(clone);
                }
            }

            assert!(!game_states.is_empty());
            // return all the possible next game states
            return Some(game_states);
        } // loop
    }

    pub fn find_holes(&self) -> Vec<(usize, Vec<usize>)> {
        let mut holes: Vec<(usize, Vec<usize>)> = Vec::with_capacity(4);
        let mut fillers: Vec<usize> = Vec::with_capacity(3);

        let piles = &self.piles;

        for hole_pile in 0..4 {
            // No hole, continue scanning
            if !piles[hole_pile].is_empty() {
                continue;
            }
            for fill_pile in 0..4 {
                if fill_pile == hole_pile {
                    continue;
                }
                if piles[fill_pile].len() >= 2 {
                    fillers.push(fill_pile);
                }
            }
            if !fillers.is_empty() {
                holes.push((hole_pile, fillers.to_vec()));
                fillers.clear();
            }
        }
        holes
    }
}
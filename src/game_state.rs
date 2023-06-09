use crate::card::*;
use crate::card_pile::CardPile;
use crate::decision::Decision;
use crate::format::{FormatType, Formattable};
use crate::game_result::GameResult;
use crate::macros::*;
use crate::misc::*;
use crate::rank::{Rank, RankOrder};
use std::cmp::Ordering;
use std::fmt::*;

#[derive(Clone, Debug)]
pub struct GameState {
    pub deck: CardPile,
    pub piles: [CardPile; 4],
    pub discard: CardPile,

    //pub initial_state: String,
    pub decision_chance: f64,
    //pub decisions: Vec<Decision>,
}

fn pile_move_card(source_pile: &mut CardPile, dest_pile: &mut CardPile) {
    let card = source_pile.pop().unwrap();
    dest_pile.push(card);
    //println!("Moved {:?}", card);
}

fn suit_rank_compare(left: &Card, right: &Card) -> Ordering {
    if left.suit == right.suit {
        let ordering = get_ordering(&left.rank, &right.rank, &RankOrder::ACE_HIGH);
        let o = ordering.unwrap_or(Ordering::Equal);
        o
    } else {
        Ordering::Equal
    }
}

struct FillHoles {
    pub total_decisions: i32,
    pub hole_fillers: Vec<(usize, Vec<usize>)>,
}

impl GameState {
    pub fn new(deck: Vec<Card>) -> GameState {
        GameState {
            //initial_state: String::new(),
            deck,
            piles: [
                Vec::with_capacity(13),
                Vec::with_capacity(13),
                Vec::with_capacity(13),
                Vec::with_capacity(13),
            ],
            discard: Vec::with_capacity(52),
            decision_chance: 1.0,
            //decisions: Vec::new(),
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
                //initial_state: self.initial_state.clone(),
                //final_state: Formattable::format_string(self, &FormatType::Emoji),
                decision_chance: self.decision_chance,
                //decisions: self.decisions.clone(),
            })
        }
    }

    pub fn move_top_card(&mut self, source_pile_index: usize, dest_pile_index: usize) {
        let piles = get_two_mut(&mut self.piles, source_pile_index, dest_pile_index);
        match piles {
            None => {}
            Some((source_pile, dest_pile)) => pile_move_card(source_pile, dest_pile),
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
                let left_card = some_or_continue!(left_pile.last());

                // scan against everything in the right half
                for right_pile in right_half.iter_mut() {
                    let right_card = some_or_continue!(right_pile.last());

                    // collide
                    let order = suit_rank_compare(left_card, right_card);
                    match order {
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
        // main
        loop {
            // collide + fill loop
            loop {
                // start with collision until there are no matching suits
                self.collide();

                // check for fillable holes
                let fill_holes: FillHoles = self.find_fillable_holes();

                // if there aren't any, we can go to the deal
                if fill_holes.total_decisions == 0 {
                    break;
                }

                // If we have exactly one hole and one pile that can fill it
                if fill_holes.total_decisions == 1 {
                    // This is an easy move!
                    let hole_index = &fill_holes.hole_fillers[0].0;
                    let fill_index = &fill_holes.hole_fillers[0].1[0];
                    self.move_top_card(*fill_index, *hole_index);

                    // go back to collide + check for holes
                    continue;
                }

                // We have to split
                let multiplier = 1.0 / fill_holes.total_decisions as f64;

                let mut game_states = Vec::with_capacity(4);
                for hole in fill_holes.hole_fillers.iter() {
                    let (hole_index, file_indices) = hole;
                    for fill_index in file_indices.iter() {
                        // let decision = Decision {
                        //     deal_count: 13_usize - (self.deck.len() / 4_usize),
                        //     fill_pile: *fill_index,
                        //     hole_pile: *hole_index,
                        // };
                        let mut clone: GameState = self.clone();
                        clone.decision_chance *= multiplier;
                        //clone.decisions.push(decision);
                        clone.move_top_card(*fill_index, *hole_index);
                        game_states.push(clone);
                    }
                }

                assert_eq!(game_states.len(), fill_holes.total_decisions as usize);
                // return all the possible next game states
                return Some(game_states);
            }

            // we have no collisions and no fillable piles

            if self.deck.is_empty() {
                // We're finished
                return None;
            }

            // deal a card to each pile
            for i in 0..4 {
                pile_move_card(&mut self.deck, &mut self.piles[i]);
            }

            // now we go back to the collide loop
        }
    }



    fn find_fillable_holes(&self) -> FillHoles {
        let mut decisions = 0;
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
                    decisions+=1;
                }
            }
            if !fillers.is_empty() {
                holes.push((hole_pile, fillers.to_vec()));
                fillers.clear();
            }
        }
        FillHoles {
            total_decisions: decisions,
            hole_fillers: holes,
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Deck: {:?}\tPileA: {:?}\tPileB: {:?}\tPileC: {:?}\tPileD: {:?}\tDiscard: {:?}",
            self.deck, self.piles[0], self.piles[1], self.piles[2], self.piles[3], self.discard
        )
    }
}

impl Formattable for GameState {
    fn format_emoji(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("deck:")?;
        self.deck.format_emoji(f)?;
        f.write_char('\n')?;
        for (i, pile) in self.piles.iter().enumerate() {
            write!(f, "pile{}:", i)?;
            pile.format_emoji(f)?;
            f.write_char('\n')?;
        }
        f.write_str("discard:")?;
        self.discard.format_emoji(f)?;
        Ok(())
    }

    fn format_short(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("deck:")?;
        self.deck.format_short(f)?;
        f.write_char('\n')?;
        for (i, pile) in self.piles.iter().enumerate() {
            write!(f, "pile{}:", i)?;
            pile.format_short(f)?;
            f.write_char('\n')?;
        }
        f.write_str("discard:")?;
        self.discard.format_short(f)?;
        Ok(())
    }

    fn format_long(&self, f: &mut Formatter<'_>) -> Result {
        self.format_short(f)
    }
}

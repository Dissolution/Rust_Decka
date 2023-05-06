#[allow(dead_code)]
#[allow(unused)]
#[allow(unused_imports)]
mod deck;
mod card;
mod face;
mod orientation;
mod rank;
mod suit;

use std::fmt;
use std::fmt::{Error, Formatter};
use std::io::ErrorKind;
use crate::card::Card;
use crate::deck::Deck;
use crate::rank::Rank;


fn main() {
    let mut game_state = GameState {
        deck: Deck::new_standard_52(),
        piles: [Vec::with_capacity(13), Vec::with_capacity(13), Vec::with_capacity(13), Vec::with_capacity(13)],
        discard: Vec::with_capacity(52),
    };
    fastrand::shuffle(&mut game_state.deck);
    for card in game_state.deck.iter() {
        println!("{:?}", card);
    }

    let results = play_to_decision(&mut game_state);
    match results {
        Some(game_states) => {
            for game_state in game_states.iter() {
                println!("Game State: {:?}", game_state)
            }
        }
        None => {
                println!("Finished");
            }
    };
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub deck: Vec<Card>,
    pub piles: [Vec<Card>; 4],
    pub discard: Vec<Card>,
}

// impl GameState {
//     fn get_pile(&self, pile_name: char) -> Option<&Vec<Card>> {
//         match pile_name {
//             'A' => {
//                 Some(&self.piles[0])
//             }
//             'B' => {
//                 Some(&self.piles[1])
//             }
//             'C' => {
//                 Some(&self.piles[2])
//             }
//             'D' => {
//                 Some(&self.piles[3])
//             }
//             _ => None,
//         }
//     }
// }


fn play_to_decision(game_state: &mut GameState) -> Option<Vec<GameState>> {
    loop {
        let deck: &mut Vec<Card> = &mut game_state.deck;
        if deck.len() < 4 {
            println!("Deck is empty");
            return Option::None;
        }

        for i in 0..4 {
            let card: Card = deck.pop().unwrap();
            let pile: &mut Vec<Card> = &mut game_state.piles[i];
            pile.push(card);
        }

        collide(game_state);

        // holes
        let holes = find_holes(&game_state.piles);

        if holes.len() == 0 {
            continue;
        }
        if holes.len() == 1 && holes[0].1.len() == 1 {
            //let mut hole_pile = game_state.get_pile(holes[0].0);
            //let mut fill_pile = game_state.get_pile(holes[0].1[0]);
        }

        // Have to split!!
        let game_states = Vec::with_capacity(4);
        for hole in holes.iter() {
            let (hole_index, file_indices) = hole;
            for fill_index in file_indices.iter() {
                let clone = game_state.clone();
                move_card()
            }
        }


        todo!();
    } // loop
}

fn find_holes(piles: &[Vec<Card>]) -> Vec<(usize, Vec<usize>)> {
    let mut holes: Vec<(usize, Vec<usize>)> = Vec::with_capacity(4);
    let mut fillers: Vec<usize> = Vec::with_capacity(3);

    for hole_pile in 0..4 {
        // No hole, continue scanning
        if piles[hole_pile].len() != 0 {
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
        if fillers.len() > 0 {
            holes.push((hole_pile, fillers.clone()));
            fillers.clear();
        }
    }
    holes
}

fn collide_compare(left: &Card, right: &Card) -> i8 {
    if left.suit == right.suit {
        let left_rank = match left.rank {
            Rank::Ace => u8::MAX,
            _ => left.rank as u8,
        };
        let right_rank = match right.rank {
            Rank::Ace => u8::MAX,
            _ => right.rank as u8,
        };
        if left_rank < right_rank {
            return -1;
        } else if left_rank > right_rank {
            return 1;
        } else {
            panic!("Duplicate card!!!");
        }
    }
    return 0;
}

fn move_card(source_pile: &mut Vec<Card>, dest_pile: &mut Vec<Card>) {
    let card = source_pile.pop().unwrap();
    dest_pile.push(card);
    println!("Moved {:?}", card);
}

fn move_card_pileindex(game_state: &mut GameState, source_index: usize, dest_index: usize) {
    game_state.piles.swap(source_index, dest_index);
}

fn collide(game_state: &mut GameState) {
    'main: loop {
        // midpoint for split
        for mid in 1..=2 {

            // split here
            let (left_half, right_half) = game_state.piles.split_at_mut(mid);

            // left pile is the last one in the left half
            let left_pile = &mut left_half.last_mut().unwrap();
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
                let c = collide_compare(&left_card, &right_card);
                if c < 0 {
                    move_card(left_pile, &mut game_state.discard);
                    continue 'main;
                } else if c > 0 {
                    move_card(right_pile, &mut game_state.discard);
                    continue 'main;
                }
            }
        }
        // done colliding
        break;
    }
}

fn get2<T>(slice: &mut [T], i: usize, j: usize) -> Result<(&mut T, &mut T), &mut T> {
    if i < j {
        let (left, right) = slice.split_at_mut(j);
        Ok((&mut left[i], &mut right[0]))
    } else if j < i {
        let (left, right) = slice.split_at_mut(i);
        Ok((&mut left[j], &mut right[0]))
    } else {
        Err(&mut slice[i])
    }
}
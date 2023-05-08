extern crate core;

mod card;
mod card_pile;
mod decision;
#[allow(dead_code)]
#[allow(unused)]
#[allow(unused_imports)]
mod deck;
mod format;
mod game_result;
mod game_state;
mod macros;
mod misc;
mod other;
mod rank;
mod suit;

use crate::deck::*;
use crate::format::{FormatType, Formattable};
//use crate::format::{FormatType, Formattable};
use crate::game_result::GameResult;
use crate::game_state::*;

fn main() {
    // get our rng
    let rng = fastrand::Rng::new();
    //with_seed(1471013u64);

    // Create the initial game state
    let mut game_state = GameState::new(Deck::new_standard_52());
    //Deck::copy_new_standard_52(&mut game_state.deck);

    // Start testing
    let mut shuffle_count = 0;
    let mut win_chance: f64 = 0.0;
    let mut perfect_chance: f64 = 0.0;

    const ITERATIONS: i32 = 10_000;
    while shuffle_count < ITERATIONS {
        // Shuffle
        rng.shuffle(&mut game_state.deck);
        shuffle_count += 1;
        //game_state.initial_state = Formattable::format_string(&game_state.deck, &FormatType::Emoji);

        // Play the entire game
        let clone = game_state.clone();
        let game_results = play_game_state(clone);

        // Process results
        for game_result in game_results.iter() {
            if game_result.win {
                win_chance += game_result.decision_chance;
                if game_result.pile_count == 4 {
                    perfect_chance += game_result.decision_chance;
                }
            }
        }

        // the next shuffle
    }

    let total_win_chance: f64 = win_chance / ITERATIONS as f64;
    let total_perfect_chance: f64 = perfect_chance / ITERATIONS as f64;
    println!(
        "Win Chance: {:.2}%\tPerfect Chance: {:.2}%",
        total_win_chance * 100.0,
        total_perfect_chance * 100.0
    );
    println!();
}

fn play_game_state(game_state: GameState) -> Vec<GameResult> {
    let mut game_results = Vec::new();

    let mut game_states = Vec::new();
    game_states.push(game_state);

    loop {
        let game_state = game_states.pop();
        if game_state.is_none() {
            //println!("Finished processing all possible game states");
            break;
        }
        let mut game_state = game_state.unwrap();

        let results = game_state.play_to_decision();
        match results {
            Some(fut_game_states) => {
                for fut_game_state in fut_game_states.into_iter() {
                    // println!("Game State: {:?}", game_state)
                    game_states.push(fut_game_state)
                }
            }
            None => {
                //let format_string = Formattable::format_string(&game_state, &FormatType::Short);
                //println!("{}", format_string);
                let result = game_state.current_result();
                //println!("{:?}", result);

                if result.is_none() {
                    panic!("WTF")
                }

                let game_result = result.unwrap();
                game_results.push(game_result);
            }
        }
    }

    //let total_chance: f64 = game_results.iter().map(|r| r.decision_chance).sum();
    //assert!(total_chance >= 0.99);
    game_results
}

#[allow(dead_code)]
#[allow(unused)]
#[allow(unused_imports)]
mod deck;
mod cards;
mod misc;
mod game;


use crate::deck::*;
use crate::game::*;
use crate::misc::*;

fn main() {
    // get our rng
    let rng = fastrand::Rng::with_seed(147u64);

    // Create the initial game state
    let mut game_state = GameState::new();
    Deck::copy_new_standard_52(&mut game_state.deck);

    // Start testing
    let mut shuffle_count = 0;
    loop {
        // Shuffle
        rng.shuffle(&mut game_state.deck);
        shuffle_count += 1;

        // Play the entire game
        let clone = game_state.clone();
        let result = play_game_state(clone);
        // Did we get anything interesting?
        let wins: Vec<&GameResult> = result
            .iter()
            .filter(|r| r.win)
            .collect();
        if !wins.is_empty() {
            println!("Shuffle #{}: {} wins, {} losses", shuffle_count, wins.len(), result.len() - wins.len());
            println!();
        }

        // try the next shuffle
    }
}

fn play_game_state(game_state: GameState) -> Vec<GameResult> {
    let mut game_results = Vec::new();

    let mut game_states = Vec::new();
    game_states.push(game_state);

    loop {
        let game_state = game_states.pop();
        if game_state.is_none() {
            println!("Finished processing all possible game states");
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
                let format_string = format!("{:?}", Fmt(|f| game_state.display(f)));
                println!("{}", format_string);
                let result = game_state.current_result();
                println!("{:?}", result);

                if result.is_none() { panic!("WTF")}

                let game_result = result.unwrap();
                game_results.push(game_result);
            }
        }
    }

    let total_chance: f64 = game_results.iter().map(|r| r.decision_chance).sum();
    assert!(total_chance >= 0.99);
    game_results
}
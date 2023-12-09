use std::collections::HashMap;
use crate::days::day_2::game::{Game, GAME_CONFIGURATION};
use crate::days::day_2::{get_game_id_from_line, get_games_from_line};
use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_2/input") {
        Ok(lines) => {
            println!(
                "Day 2: 'Finding valid games' - {}",
                calculate_possible_games(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn is_game_possible(game: Game) -> bool {
    if game.red > GAME_CONFIGURATION.red
        || game.green > GAME_CONFIGURATION.green
        || game.blue > GAME_CONFIGURATION.blue
    {
        return false;
    }

    true
}

fn calculate_possible_games(lines: Vec<String>) -> u32 {
    let mut result: u32 = 0;

    // Checking all lines
    'lines: for line in lines {
        let id: u32 = get_game_id_from_line(&line);
        let games: HashMap<u8, Game> = get_games_from_line(&line);

        // Checking all games per line. If one is negative -> don't add to result
        for (_index, game) in games {
            if !is_game_possible(game) {
                continue 'lines;
            }
        }

        // Making sure to add the id of possible game
        result += id;
    }

    result
}
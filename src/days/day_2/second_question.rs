use crate::days::day_2::game::Game;
use crate::days::day_2::get_games_from_line;
use crate::days::read_input_into_vector;
use std::collections::HashMap;

pub fn solution() {
    match read_input_into_vector("./src/days/day_2/input") {
        Ok(lines) => {
            println!(
                "Day 2: 'Finding the power' - {}",
                calculate_power_of_games(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_power_of_game(game: Game) -> u32 {
    game.red as u32 * game.green as u32 * game.blue as u32
}

fn get_min_game(games: HashMap<u8, Game>) -> Game {
    let mut min_game = Game::default();

    for (_index, game) in games {
        if game.red > min_game.red {
            min_game.red = game.red;
        }

        if game.green > min_game.green {
            min_game.green = game.green;
        }

        if game.blue > min_game.blue {
            min_game.blue = game.blue;
        }
    }

    min_game
}

fn calculate_power_of_games(lines: Vec<String>) -> u32 {
    let mut result: u32 = 0;

    // Checking all lines
    for line in lines {
        let games = get_games_from_line(&line);
        let min_game = get_min_game(games);
        let power_of_line: u32 = calculate_power_of_game(min_game);

        // Making sure to add the id of possible game
        result += power_of_line;
    }

    result
}

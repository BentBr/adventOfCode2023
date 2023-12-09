mod game;

use game::Game;
use game::GAME_CONFIGURATION;
use std::collections::HashMap;
use std::ops::Add;

mod first_question;
mod second_question;

pub fn solutions() {
    first_question::solution();
    second_question::solution();
}

// Configuration: 12 red cubes, 13 green cubes, and 14 blue cubes
//Which games are possible?
// Add those ids up (as integer)

fn get_game_id_from_line(games_line: &str) -> u32 {
    let cleaned_id: u32;
    // Structure is given as fixed
    if let Some(id) = games_line.split(' ').nth(1) {
        let cleaned_id_string = id.trim_end_matches(':');

        cleaned_id = cleaned_id_string.parse::<u32>().unwrap();
    } else {
        panic!("id not found in game line: {}", games_line);
    }

    cleaned_id
}

fn get_games_from_line(games_line: &str) -> HashMap<u8, Game> {
    let mut games_map: HashMap<u8, Game> = HashMap::new();

    // Checking one line for games
    //Game 6: 3 green, 7 blue, 5 red; 3 green, 6 red; 11 blue, 6 red, 1 green
    if let Some(games) = games_line.split(':').nth(1) {
        // Checking game line for single games
        //  3 green, 7 blue, 5 red; 3 green, 6 red; 11 blue, 6 red, 1 green
        for game_string in games.split(';').into_iter() {
            let mut game: Game = Game::default();

            // Breaking down the games. Checking for colours and fallback to 0 if one is not given
            //  3 green, 7 blue, 5 red
            for colour in game_string.split(',').into_iter() {
                // We can have spaces at the beginning and at the end :(
                let sanitized_colour = colour.trim_start_matches(' ').trim_end_matches(' ');
                let colour_split = sanitized_colour.split(' ');

                let colour_int: u8;
                let colour_string: &str;

                if let Some(colour_split) = colour_split.clone().nth(0) {
                    colour_int = colour_split.parse::<u8>().unwrap();
                } else {
                    panic!("Could not find colour's int for {}", colour)
                }

                if let Some(colour_split) = colour_split.clone().nth(1) {
                    colour_string = colour_split
                } else {
                    panic!("Could not find colour's int for {}", colour)
                }

                // Assigning the correct colours
                match colour_string {
                    "red" => game.red = colour_int,
                    "green" => game.green = colour_int,
                    "blue" => game.blue = colour_int,
                    _ => panic!("Could not match the colour {}", colour_string),
                }
            }

            let games_len = games_map.len();
            games_map.insert(games_len as u8, game);
        }
    } else {
        panic!("games not found in game line: {}", games_line);
    }

    games_map
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

fn calculate_games(lines: Vec<String>) -> u32 {
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

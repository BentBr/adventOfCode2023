use super::{create_race_vector_from_lines, Race};
use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_6/input") {
        Ok(lines) => {
            println!(
                "Day 6: 'Winning possibilities' - {}",
                calculate_winning_possibilities(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_winning_possibilities(lines: Vec<String>) -> u32 {
    let races = create_race_vector_from_lines(lines);
    let mut possibilities: u32 = 1;

    for race in races {
        possibilities *= calculate_possibilities_per_race(race);
    }

    possibilities
}

fn calculate_possibilities_per_race(race: Race) -> u32 {
    let mut possibilities = 0;

    for time in 1..race.time {
        let travel_distance = (race.time - time) * time;

        if travel_distance > race.distance {
            possibilities += 1;
        }
    }

    possibilities
}

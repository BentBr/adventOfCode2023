use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_2/input") {
        Ok(lines) => {
            println!(
                "Day 2: 'Finding valid games' - {}",
                crate::days::day_2::calculate_games(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}
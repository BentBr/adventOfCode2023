use super::calculate_winning_possibilities;
use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_6/input") {
        Ok(lines) => {
            println!(
                "Day 6: 'Winning possibilities' - {}",
                calculate_winning_possibilities(lines, false)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

use crate::days::day_6::calculate_winning_possibilities;
use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_6/input") {
        Ok(lines) => {
            println!(
                "Day 6: ''Winning possibilities (spaceless)' - {}",
                calculate_winning_possibilities(lines, true)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

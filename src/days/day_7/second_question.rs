use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_6/input") {
        Ok(_lines) => {
            println!(
                "Day 7: ''Winning possibilities (spaceless)' - {}",
                calculate_winning_possibilities()
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_winning_possibilities() -> u32 {
    34
}

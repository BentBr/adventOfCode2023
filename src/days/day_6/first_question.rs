use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_5/test_input") {
        Ok(lines) => {
            println!(
                "Day 5: 'Scratchcard values' - {}",
                calculate_scratchcard_sum(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

use super::get_card_numbers_from_line;
use super::get_count_of_matching_winning_numbers;
use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_4/input") {
        Ok(lines) => {
            println!(
                "Day 4: 'Scratchcard values' - {}",
                calculate_scratchcard_sum(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_scratchcard_sum(lines: Vec<String>) -> u32 {
    let mut returning_sum: u32 = 0;

    for (_, line) in lines.iter().enumerate() {
        let winning_numbers: Vec<u8> = get_card_numbers_from_line(line, true);
        let scratched_numbers: Vec<u8> = get_card_numbers_from_line(line, false);

        let count_of_matching_winning_numbers: u32 =
            get_count_of_matching_winning_numbers(winning_numbers, scratched_numbers);

        if count_of_matching_winning_numbers > 0 {
            let calculated_points: u32 = (2u32).pow(count_of_matching_winning_numbers - 1);

            returning_sum += calculated_points;
        }
    }

    returning_sum
}

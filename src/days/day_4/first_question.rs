use crate::days::read_input_into_vector;
use std::collections::HashSet;

pub fn solution() {
    match read_input_into_vector("./src/days/day_4/input") {
        Ok(lines) => {
            println!(
                "Day 3: 'Scratchcard values' - {}",
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

fn get_count_of_matching_winning_numbers(
    winning_numbers: Vec<u8>,
    scratched_numbers: Vec<u8>,
) -> u32 {
    let set1: HashSet<_> = winning_numbers.into_iter().collect();
    let set2: HashSet<_> = scratched_numbers.into_iter().collect();

    let intersection: HashSet<u8> = set1.intersection(&set2).cloned().collect();
    let intersection_vec: Vec<u8> = intersection.into_iter().collect();

    intersection_vec.len() as u32
}

fn get_card_numbers_from_line(line: &str, left: bool) -> Vec<u8> {
    let mut card_numbers: Vec<u8> = Default::default();

    let nth: usize = match left {
        true => 0,
        false => 1,
    };

    if let Some(numbers) = line.split(':').nth(1) {
        let mut trimmed_numbers = numbers.trim_end_matches(' ');
        trimmed_numbers = trimmed_numbers.trim_start_matches(' ');

        if let Some(number_side) = trimmed_numbers.split('|').nth(nth) {
            let mut trimmed_number_side = number_side.trim_end_matches(' ');
            trimmed_number_side = trimmed_number_side.trim_start_matches(' ');

            for found_card_number in trimmed_number_side.split(' ') {
                if found_card_number.is_empty() {
                    continue;
                }

                let number = found_card_number.trim_matches(' ');
                let number_int: u8 = number.parse::<u8>().unwrap();

                card_numbers.push(number_int);
            }
        }
    }

    card_numbers
}

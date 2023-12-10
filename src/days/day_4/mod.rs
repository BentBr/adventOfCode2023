use std::collections::HashSet;
mod first_question;
mod second_question;

pub fn solutions() {
    first_question::solution();
    second_question::solution();
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

fn get_id_from_line(line: &str) -> u8 {
    let mut id: u8 = 0;

    if let Some(game_id_string) = line.split(':').next() {
        if let Some(game_id) = game_id_string.split(' ').nth(1) {
            id = game_id.parse::<u8>().unwrap();
        }
    }

    if id == 0 {
        panic!("Did not parse a game card id properly: {}", line);
    }

    id
}

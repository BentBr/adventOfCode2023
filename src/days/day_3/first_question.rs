use super::get_numbers_from_line;
use super::get_symbols_from_line;
use crate::days::read_input_into_vector;
use std::collections::HashMap;

pub fn solution() {
    match read_input_into_vector("./src/days/day_3/input") {
        Ok(lines) => {
            println!(
                "Day 3: 'Schematic sum' - {}",
                calculate_schematic_sum(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_schematic_sum(lines: Vec<String>) -> u32 {
    let mut returning_sum: u32 = 0;

    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut previous_line: &String = &"".to_string();
        let mut next_line: &String = &"".to_string();

        if index != 0 {
            previous_line = lines.get(index - 1).unwrap();
        }
        if index + 1 != lines.len() {
            next_line = lines.get(index + 1).unwrap();
        }

        let valid_part_numbers: Vec<u32> = get_valid_part_numbers(line, previous_line, next_line);

        let valid_sum: u32 = valid_part_numbers.iter().sum();
        returning_sum += valid_sum;
    }

    returning_sum
}

fn get_valid_part_numbers(current_line: &str, previous_line: &str, next_line: &str) -> Vec<u32> {
    let numbers_map: HashMap<u8, u16> = get_numbers_from_line(current_line);
    let mut valid_numbers: Vec<u32> = Default::default();

    let symbol_maps: [HashMap<u8, char>; 3] = [
        get_symbols_from_line(previous_line),
        get_symbols_from_line(current_line),
        get_symbols_from_line(next_line),
    ];

    for (index, number_to_check) in numbers_map {
        for symbols_of_line in &symbol_maps {
            if !symbols_of_line.is_empty()
                && is_part_number_relevant(index, number_to_check, symbols_of_line.clone())
            {
                valid_numbers.push(number_to_check as u32);
                break;
            }
        }
    }

    valid_numbers
}

fn is_part_number_relevant(
    index_of_number: u8,
    number: u16,
    symbols_in_line: HashMap<u8, char>,
) -> bool {
    let min_index = index_of_number;
    let max_index = index_of_number + number.to_string().len() as u8 - 1;

    for (symbol_index, _) in symbols_in_line {
        // We are checking for the range of those indices (depending on the len() of the number)
        let within_range = symbol_index >= min_index && symbol_index <= max_index + 1;
        let within_overflow_range =
            min_index > 0 && symbol_index >= min_index - 1 && symbol_index <= max_index + 1;

        if within_range || within_overflow_range {
            return true;
        }
    }

    false
}

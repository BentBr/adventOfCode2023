use crate::days::read_input_into_vector;
use std::collections::HashMap;

pub fn solution() {
    match read_input_into_vector("./src/days/day_3/input") {
        Ok(lines) => {
            println!(
                "Day 2: 'Schematic sum' - {}",
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

fn get_valid_part_numbers(line: &String, previous_line: &String, next_line: &String) -> Vec<u32> {
    let numbers_map: HashMap<u8, u16> = get_numbers_from_line(line);
    let mut valid_numbers: Vec<u32> = Default::default();

    let symbol_maps = [
        get_symbols_from_line(previous_line),
        get_symbols_from_line(line),
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

// Returns a HashMap of indices where certain numbers start and the number it self.
fn get_numbers_from_line(line: &str) -> HashMap<u8, u16> {
    let mut numbers_map: HashMap<u8, u16> = Default::default();
    let mut current_number_string: String = "".to_string();

    for (index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            current_number_string.push(char);
        } else if !current_number_string.is_empty() {
            let new_index = index - current_number_string.len();

            numbers_map.insert(
                new_index as u8,
                current_number_string.clone().parse::<u16>().unwrap(),
            );
            current_number_string = "".to_string();
        }
    }

    numbers_map
}

// Returns a HashMap of indices where certain symbols reside inside a line
fn get_symbols_from_line(line: &str) -> HashMap<u8, char> {
    let mut symbols_map: HashMap<u8, char> = Default::default();

    for (index, char) in line.chars().enumerate() {
        if !char.is_numeric() && char != ".".chars().next().unwrap_or_default() {
            symbols_map.insert(index as u8, char);
        }
    }

    symbols_map
}

fn is_part_number_relevant(
    index_of_number: u8,
    number: u16,
    symbols_in_line: HashMap<u8, char>,
) -> bool {
    let min_index = index_of_number;
    let max_index = index_of_number + number.to_string().len() as u8 - 1;

    for (symbol_index, _symbol) in symbols_in_line {
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

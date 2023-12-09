use super::get_numbers_from_line;
use super::get_symbols_from_line;
use crate::days::read_input_into_vector;
use std::collections::HashMap;

pub fn solution() {
    match read_input_into_vector("./src/days/day_3/input") {
        Ok(lines) => {
            println!(
                "Day 3: 'Gear ratio sum' - {}",
                calculate_gear_ratio_sum(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_gear_ratio_sum(lines: Vec<String>) -> u32 {
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

        let gear_ratios: Vec<u32> = get_ratios_for_line(line, previous_line, next_line);
        let gear_ratio_sum: u32 = gear_ratios.iter().sum();
        returning_sum += gear_ratio_sum;
    }

    returning_sum
}

fn get_ratios_for_line(line: &str, previous_line: &str, next_line: &str) -> Vec<u32> {
    let mut gear_ratios: Vec<u32> = Default::default();
    let symbols = get_symbols_from_line(line);
    let gear_candidates = get_gear_candidates_from_symbols(symbols);

    for (gear_candidate_index, _) in gear_candidates {
        let gear = get_gear_from_candidate(gear_candidate_index, previous_line, line, next_line);

        if !gear.is_empty() {
            gear_ratios.push(calculate_gear_ratio(gear))
        }
    }

    gear_ratios
}

fn get_gear_from_candidate(
    gear_candidate_index: u8,
    previous_line: &str,
    current_line: &str,
    next_line: &str,
) -> Vec<u16> {
    let mut return_gear_vector: Vec<u16> = Default::default();

    let numbers_line_map: [HashMap<u8, u16>; 3] = [
        get_numbers_from_line(previous_line),
        get_numbers_from_line(current_line),
        get_numbers_from_line(next_line),
    ];

    for numbers_of_line in numbers_line_map {
        for (number_index, number) in &numbers_of_line {
            if !numbers_of_line.is_empty()
                && is_part_number_relevant(&gear_candidate_index, number_index, number)
            {
                return_gear_vector.push(*number);
            }
        }
    }

    // Checking length: only exact 2 are allowed
    if return_gear_vector.len() != 2 {
        let defaulting_zero: Vec<u16> = Default::default();
        return defaulting_zero;
    }

    return_gear_vector
}

fn calculate_gear_ratio(gear: Vec<u16>) -> u32 {
    if gear.len() > 2 {
        panic!("This is not a gear (too many part numbers): {:?}", gear)
    }

    let first_part: u32 = *gear.get(0).unwrap() as u32;
    let second_part: u32 = *gear.get(1).unwrap() as u32;

    let result: u32 = first_part * second_part;

    result
}

fn get_gear_candidates_from_symbols(symbols: HashMap<u8, char>) -> HashMap<u8, char> {
    let mut return_map: HashMap<u8, char> = Default::default();

    for (index, symbol) in symbols {
        if symbol == "*".chars().next().unwrap() {
            return_map.insert(index, symbol);
        }
    }

    return_map
}

fn is_part_number_relevant(index_of_gear_candidate: &u8, number_index: &u8, number: &u16) -> bool {
    let min_index: &u8 = number_index;
    let max_index: u8 = number_index + number.to_string().len() as u8 - 1;

    // We are checking for the range of those indices (depending on the len() of the number)
    let within_range =
        index_of_gear_candidate >= min_index && index_of_gear_candidate <= &(max_index + 1);
    let within_overflow_range = min_index > &0
        && index_of_gear_candidate >= &(min_index - 1)
        && index_of_gear_candidate <= &(max_index + 1);

    if within_range || within_overflow_range {
        return true;
    }

    false
}

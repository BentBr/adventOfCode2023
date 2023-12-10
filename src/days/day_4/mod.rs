use std::collections::HashMap;

mod first_question;
mod second_question;

pub fn solutions() {
    first_question::solution();
    second_question::solution();
}

// Returns a HashMap of indices where certain numbers start and the number it self.
fn get_numbers_from_line(line: &str) -> HashMap<u8, u16> {
    let mut numbers_map: HashMap<u8, u16> = Default::default();
    let mut current_number_string: String = "".to_string();

    for (index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            current_number_string.push(char);

            // Edge case for last one
            if index == line.chars().count() - 1 {
                // Here we are adding 1 due to the fact of not yet being in next loop (will never be)
                let new_index = index - current_number_string.len() + 1;

                numbers_map.insert(
                    new_index as u8,
                    current_number_string.clone().parse::<u16>().unwrap(),
                );
            }
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

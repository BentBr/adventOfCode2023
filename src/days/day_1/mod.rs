use std::collections::HashMap;

mod first_question;
mod second_question;

pub fn solutions() {
    first_question::solution();
    second_question::solution();
}

// Calculator of this quiz
fn calibration_values(lines: Vec<String>, spelled_digit: bool) -> u32 {
    let mut result: u32 = 0;

    // Iterating over every every line given in file
    for line in lines {
        let mut resulting_number: String = "".to_string();

        let mut found_numbers_at_place: HashMap<u8, u8> = HashMap::new();

        // Iterating over every char of that string
        for (index, char) in line.chars().enumerate() {
            // Checking for number
            if char.is_numeric() {
                // Fixing the conversion issues by adding " - b'0"
                found_numbers_at_place.insert(index as u8, char as u8 - b'0');
            }
        }

        if spelled_digit {
            // Merging both HashMaps together (with the spelled one)
            found_numbers_at_place.extend(spelled_digit_matcher(&line));
        }

        let smallest_key = found_numbers_at_place.keys().min();
        let char_by_smallest_index = match smallest_key {
            Some(&key) => {
                let value = found_numbers_at_place.get(&key); // Get the value associated with the minimum key
                match value {
                    Some(val) => val,
                    None => panic!("Something went wrong during HashMap checking for smallest key"),
                }
            }
            None => panic!("Something went wrong during HashMap checking for smallest key"),
        };

        // Fallback if only one has been found
        if found_numbers_at_place.len() < 2 {
            // Fixing the conversion issues by adding " + b'0'"
            resulting_number.push((*char_by_smallest_index + b'0') as char);
            resulting_number.push((*char_by_smallest_index + b'0') as char);
        // Finding the smallest and the biggest index
        } else {
            let biggest_key = found_numbers_at_place.keys().max();
            let char_by_biggest_index = match biggest_key {
                Some(&key) => {
                    let value = found_numbers_at_place.get(&key); // Get the value associated with the minimum key
                    match value {
                        Some(val) => val,
                        None => {
                            panic!("Something went wrong during HashMap checking for smallest key")
                        }
                    }
                }
                None => panic!("Something went wrong during HashMap checking for smallest key"),
            };

            // Fixing the conversion issues by adding " + b'0'"
            resulting_number.push((*char_by_smallest_index + b'0') as char);
            resulting_number.push((*char_by_biggest_index + b'0') as char);
        }

        match resulting_number.parse::<u32>() {
            Ok(parsed_num) => {
                result += parsed_num;
            }
            Err(err) => {
                println!("Error parsing: {:?} {}", &resulting_number, err);
            }
        }
    }

    result
}

// Check the giving string if a spelled digit is present
// if found multiple, the bool low indicates if the lower or the higher one is to be returned
fn spelled_digit_matcher(string_to_check: &str) -> HashMap<u8, u8> {
    let mut numbers_map: HashMap<&str, u8> = HashMap::new();
    let mut found_at_place: HashMap<u8, u8> = HashMap::new();

    numbers_map.insert("one", 1);
    numbers_map.insert("two", 2);
    numbers_map.insert("three", 3);
    numbers_map.insert("four", 4);
    numbers_map.insert("five", 5);
    numbers_map.insert("six", 6);
    numbers_map.insert("seven", 7);
    numbers_map.insert("eight", 8);
    numbers_map.insert("nine", 9);

    for (spelled_digit, digit) in numbers_map.iter() {
        if let Some(value) = string_to_check.find(spelled_digit) {
            found_at_place.insert(value as u8, *digit);
        }
    }

    for (spelled_digit, digit) in numbers_map.iter() {
        if let Some(rvalue) = string_to_check.rfind(spelled_digit) {
            found_at_place.insert(rvalue as u8, *digit);
        }
    }

    found_at_place
}

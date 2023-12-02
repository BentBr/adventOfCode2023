use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn solution() {
    match read_input_into_vector() {
        Ok(lines) => {
            println!(
                "Day 1: 'calibration values' - {}",
                calibration_values(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn read_input_into_vector() -> io::Result<Vec<String>> {
    let file = File::open("./src/days/day_1/input")?;
    let reader = io::BufReader::new(&file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

// Calculator of this quiz
fn calibration_values(lines: Vec<String>) -> u32 {
    let mut result: u32 = 0;

    // Having a little store for the question if already a char has been found
    let mut have_got_first: bool;

    // Iterating over every every line given
    for line in lines {
        have_got_first = false;
        let mut resulting_number: String = "".to_string();

        // Iterating over every char of that string
        for char in line.chars() {
            // Checking for number
            if char.is_numeric() {
                if !have_got_first {
                    resulting_number = char.to_string();

                    have_got_first = true;
                } else {
                    // We want to replace the 2nd number every time it occurs again one
                    if resulting_number.len() == 2 {
                        // Only if already 2 exist the latter must be removed
                        resulting_number.pop();
                    }
                    resulting_number.push(char);
                }
            }
        }

        // Fallback if only one has been found
        if resulting_number.len() < 2 {
            if let Some(first_char) = resulting_number.chars().next() {
                resulting_number.push(first_char); // Append the first character
            }
        }

        match resulting_number.parse::<u32>() {
            Ok(parsed_num) => {
                result += parsed_num;
            }
            Err(err) => {
                println!("Error parsing: {}", err);
            }
        }
    }

    result
}

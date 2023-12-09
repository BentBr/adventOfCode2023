use super::super::read_input_into_vector;
use super::calibration_values;

pub fn solution() {
    match read_input_into_vector("./src/days/day_1/input") {
        Ok(lines) => {
            println!(
                "Day 1: 'calibration values' - {}",
                calibration_values(lines, false)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

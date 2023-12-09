use super::calibration_values;
use super::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector() {
        Ok(lines) => {
            println!(
                "Day 1: 'calibration values spelled digits' - {}",
                calibration_values(lines, true)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

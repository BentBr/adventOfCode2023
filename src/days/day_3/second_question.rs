use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_2/input") {
        Ok(_lines) => {
            println!("Day 2: 'Finding the power' - testy");
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

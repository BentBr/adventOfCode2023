use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_5/test_input") {
        Ok(_lines) => {
            println!(
                "Day 4: 'Scratch card amount via iterator' - {}",
                test()
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn test() -> u8 {
    45
}
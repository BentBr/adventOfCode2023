use crate::days::read_input_into_vector;

pub fn solution() {
    match read_input_into_vector("./src/days/day_7/test_input") {
        Ok(lines) => {
            println!("Day 7: 'Ranking calculated' - {}", calculate_ranking(lines));
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_ranking(_lines: Vec<String>) -> u32 {
    45
}

use std::fs::File;
use std::io;
use std::io::BufRead;

pub mod day_1;
pub mod day_2;

fn read_input_into_vector(source: &str) -> io::Result<Vec<String>> {
    let file = File::open(source)?;
    let reader = io::BufReader::new(&file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

mod first_question;
mod second_question;

pub fn solutions() {
    first_question::solution();
    second_question::solution();
}

#[derive(Debug)]
struct Race {
    time: u16,
    distance: u16,
}

fn create_race_vector_from_lines(lines: Vec<String>) -> Vec<Race> {
    let times = lines.first().unwrap();
    let distances = lines.get(1).unwrap();
    let mut races: Vec<Race> = Default::default();

    let timing_values = get_values_from_string(times);
    let distance_values = get_values_from_string(distances);

    if timing_values.len() != distance_values.len() {
        panic!(
            "Your input is not homogeneous: {:?} {:?}",
            timing_values, distance_values
        );
    }

    for index in 0..timing_values.len() {
        let time: &u16 = timing_values.get(index).unwrap();
        let distance: &u16 = distance_values.get(index).unwrap();

        let race = Race::new(*time, *distance);

        races.push(race);
    }

    races
}

fn get_values_from_string(line: &str) -> Vec<u16> {
    let mut return_vec: Vec<u16> = Default::default();

    if let Some(mut line_string) = line.split(':').nth(1) {
        line_string = line_string.trim_start_matches(' ').trim_end_matches(' ');

        let values = line_string.split(' ');
        for value in values {
            let value_int = value.parse::<u16>().unwrap();
            return_vec.push(value_int);
        }
    }

    return_vec
}

impl Race {
    fn new(time: u16, distance: u16) -> Race {
        let race: Race = Race { time, distance };
        race
    }
}

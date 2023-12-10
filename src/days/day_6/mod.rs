mod first_question;
mod second_question;

pub fn solutions() {
    first_question::solution();
    second_question::solution();
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn create_race_vector_from_lines(lines: Vec<String>, spaceless: bool) -> Vec<Race> {
    let times = lines.first().unwrap();
    let distances = lines.get(1).unwrap();
    let mut races: Vec<Race> = Default::default();

    let timing_values = get_values_from_string(times, spaceless);
    let distance_values = get_values_from_string(distances, spaceless);

    if timing_values.len() != distance_values.len() {
        panic!(
            "Your input is not homogeneous: {:?} {:?}",
            timing_values, distance_values
        );
    }

    for index in 0..timing_values.len() {
        let time: &u64 = timing_values.get(index).unwrap();
        let distance: &u64 = distance_values.get(index).unwrap();

        let race = Race::new(*time, *distance);

        races.push(race);
    }

    races
}

fn get_values_from_string(line: &str, spaceless: bool) -> Vec<u64> {
    let mut return_vec: Vec<u64> = Default::default();

    if let Some(mut line_string) = line.split(':').nth(1) {
        line_string = line_string.trim_start_matches(' ').trim_end_matches(' ');

        if spaceless {
            let value: String = line_string.replace(' ', "");
            let value_int: u64 = value.parse::<u64>().unwrap();
            return_vec.push(value_int);
        } else {
            let values = line_string.split(' ');
            for value in values {
                let value_int = value.parse::<u64>().unwrap();
                return_vec.push(value_int);
            }
        }
    }

    return_vec
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        let race: Race = Race { time, distance };
        race
    }
}

fn calculate_winning_possibilities(lines: Vec<String>, spaceless: bool) -> u64 {
    let races = create_race_vector_from_lines(lines, spaceless);
    let mut possibilities: u64 = 1;

    for race in races {
        possibilities *= calculate_possibilities_per_race(race);
    }

    possibilities
}

fn calculate_possibilities_per_race(race: Race) -> u64 {
    let mut possibilities = 0;

    for time in 1..race.time {
        let travel_distance = (race.time - time) * time;

        if travel_distance > race.distance {
            possibilities += 1;
        }
    }

    possibilities
}

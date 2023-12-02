use super::days;

pub fn get_solution(day: u8) {
    println!("Chosen solution for day '{}'...", day);

    load_solution(day);
}

pub fn get_all_solutions() {
    println!("Getting all solutions...")
}

fn load_solution(day: u8) {
    // Dynamically load the appropriate solutions based on the provided day
    match day {
        1 => days::day_1::solutions(),
        2 => days::day_2::solutions(),
        _ => {
            println!("No solutions available for day {} (yet)", day);
        }
    }
}

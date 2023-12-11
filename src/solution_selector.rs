use super::days;

pub fn get_solution(day: u8) {
    // Defining green colour
    println!("\u{001b}[32mChosen solution for day '{}'...", day);

    load_solution(day);
}

pub fn get_all_solutions() {
    // Defining green colour
    println!("\u{001b}[32mGetting all solutions...");

    for day in 1..=24 {
        load_solution(day)
    }
}

fn load_solution(day: u8) {
    // Dynamically load the appropriate solutions based on the provided day
    // Defining red colour
    print!("\u{001b}[31m");
    match day {
        1 => days::day_1::solutions(),
        2 => days::day_2::solutions(),
        3 => days::day_3::solutions(),
        4 => days::day_4::solutions(),
        6 => days::day_6::solutions(),
        7 => days::day_7::solutions(),
        _ => {
            // Resetting the colour
            print!("\u{001b}[0m");
            // Red again (just because I can
            println!("\u{001b}[31mNo solutions available for day {} (yet)", day);
        }
    }
}

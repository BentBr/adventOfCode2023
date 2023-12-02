mod days;
mod solution_selector;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional day for the solution(s) as unsigned
    #[arg(short, long = "day")]
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();
    println!("Solutions of the Advent of Code 2023:");

    match args.day {
        Some(value) => solution_selector::get_solution(value),
        None => solution_selector::get_all_solutions(),
    }
}

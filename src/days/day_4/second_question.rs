use super::get_card_numbers_from_line;
use super::get_count_of_matching_winning_numbers;
use super::get_id_from_line;
use crate::days::read_input_into_vector;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
struct Card {
    _id: u8,
    amount: u32,
    winning_numbers: Vec<u8>,
    scratched_numbers: Vec<u8>,
}

pub fn solution() {
    match read_input_into_vector("./src/days/day_4/input") {
        Ok(lines) => {
            println!(
                "Day 4: 'Scratch card amount via iterator' - {}",
                calculate_scratching_card_iterator(lines)
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_scratching_card_iterator(lines: Vec<String>) -> u32 {
    let mut game_cards: Vec<Card> = vec![];

    // Getting all lines into scratching cards
    for line in lines {
        game_cards.push(create_card_from_line(line.as_str()));
    }

    // Iterating over the cards and updating the Card struct for each winning
    let different_game_cards = (game_cards.len() - 1) as u8;
    for card_id in 0..different_game_cards {
        // Getting the amount of winnings
        let card: &Card = game_cards.get(card_id as usize).unwrap();

        let winning_amount = get_count_of_matching_winning_numbers(
            card.winning_numbers.clone(),
            card.scratched_numbers.clone(),
        );

        // Used for multiple adds if already a clone happened
        let card_clones: u32 = card.amount;

        // Adding + 1 for each following card
        for win in 0..winning_amount + 1 {
            if win == 0 {
                continue;
            }
            let win_str = win.to_string();

            let parsed_win: Result<usize, ParseIntError> = win_str.parse::<usize>();
            let next_index: usize = match parsed_win {
                Ok(parsed) => usize::from(card_id) + parsed,
                Err(e) => {
                    panic!("Error parsing win: {:?}", e);
                }
            };

            //let next_index: usize = usize::from(card_id) + next_index;
            let card: &mut Card = game_cards.get_mut(next_index).unwrap();

            card.add_clone_to_card(card_clones);
        }
    }

    sum_cards(game_cards)
}

fn create_card_from_line(line: &str) -> Card {
    let id = get_id_from_line(line);
    let winning_numbers: Vec<u8> = get_card_numbers_from_line(line, true);
    let scratched_numbers: Vec<u8> = get_card_numbers_from_line(line, false);
    let card: Card = Card::new(id, winning_numbers, scratched_numbers);

    card
}

fn sum_cards(game_cards: Vec<Card>) -> u32 {
    let mut card_sum: u32 = 0;

    for card in game_cards {
        card_sum += card.amount;
    }

    card_sum
}

impl Card {
    fn new(id: u8, winning_numbers: Vec<u8>, scratched_numbers: Vec<u8>) -> Card {
        let card: Card = Card {
            _id: id - 1,
            amount: 1,
            winning_numbers,
            scratched_numbers,
        };

        card
    }

    fn add_clone_to_card(&mut self, amount: u32) -> &mut Card {
        self.amount += amount;

        self
    }
}

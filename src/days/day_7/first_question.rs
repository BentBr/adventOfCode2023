use crate::days::day_7::{compare_hand_ranking, create_hand_from_string, get_bid_from_string};
use Card::{Eight, Five, Four, Nine, Seven, Six, Three, Two, A, J, K, Q, T};

use crate::days::read_input_into_vector;
use std::cmp::Ordering;

#[derive(Debug, PartialOrd, PartialEq, Hash, Eq)]
pub(crate) enum Card {
    A = 12,
    K = 11,
    Q = 10,
    J = 9,
    T = 8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five = 3,
    Four = 2,
    Three = 1,
    Two = 0,
}

impl Card {
    pub(crate) fn get_value(&self) -> u8 {
        match self {
            A => 12,
            K => 11,
            Q => 10,
            J => 9,
            T => 8,
            Nine => 7,
            Eight => 6,
            Seven => 5,
            Six => 4,
            Five => 3,
            Four => 2,
            Three => 1,
            Two => 0,
        }
    }
}

pub fn solution() {
    match read_input_into_vector("./src/days/day_7/input") {
        Ok(lines) => {
            println!("Day 7: 'Ranking calculated' - {}", calculate_ranking(lines));
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn calculate_ranking(lines: Vec<String>) -> u32 {
    let mut sorted_hands: Vec<(usize, u16)> = Vec::new();
    let mut all_hands: Vec<Vec<Card>> = Vec::new();

    for line in lines {
        let bid: u16 = get_bid_from_string(&line);
        let hand: Vec<Card> = create_hand_from_string(line.as_str());

        // Store the hand in a separate vector
        all_hands.push(hand);

        // Store the index of the hand and bid in the sorted_hands vector
        sorted_hands.push((all_hands.len() - 1, bid));
    }

    sorted_hands.sort_by(|a, b| {
        let hand_a = &all_hands[a.0];
        let hand_b = &all_hands[b.0];
        if compare_hand_ranking(hand_a, hand_b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut ranking_calculation = 0;
    let number_of_hands = sorted_hands.len() as u32;

    for (index, (_, bid)) in sorted_hands.iter().enumerate() {
        ranking_calculation += *bid as u32 * (number_of_hands - index as u32);
    }

    ranking_calculation
}

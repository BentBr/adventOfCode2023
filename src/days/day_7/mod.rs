use crate::days::day_7::Card::{Eight, Five, Four, Nine, Seven, Six, Three, Two, A, J, K, Q, T};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

mod first_question;
mod second_question;
mod tests;

pub fn solutions() {
    first_question::solution();
    second_question::solution();
}

#[derive(Debug, PartialOrd, PartialEq, Hash, Eq)]
enum Card {
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
    #[allow(dead_code)]
    fn get_value(&self) -> u8 {
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

#[allow(unused)]
fn match_char_against_value() {}

fn is_five_of_a_kind(hand: &[Card]) -> bool {
    let unique_cards: HashSet<_> = hand.iter().collect();

    if unique_cards.len() == 1 {
        return true;
    }
    false
}

fn is_four_of_a_kind(hand: &[Card]) -> bool {
    let mut freq: HashMap<&Card, u8> = HashMap::new();
    for card in hand {
        *freq.entry(card).or_insert(0) += 1;
    }
    for (_, count) in freq {
        if count == 4 {
            return true;
        }
    }

    false
}

fn is_full_house(hand: &[Card]) -> bool {
    let mut freq: HashMap<&Card, u8> = HashMap::new();
    for card in hand {
        *freq.entry(card).or_insert(0) += 1;
    }

    let mut three_of_a_kind = None;
    let mut pair = None;
    for (card, count) in freq {
        if count == 3 {
            three_of_a_kind = Some(card);
        } else if count == 2 {
            pair = Some(card);
        }
    }
    if three_of_a_kind.is_some() && pair.is_some() {
        return true;
    }

    false
}

fn is_three_of_a_kind(hand: &[Card]) -> bool {
    let unique_cards: HashSet<_> = hand.iter().collect();

    let mut freq: HashMap<&Card, u8> = HashMap::new();
    for card in hand {
        *freq.entry(card).or_insert(0) += 1;
    }

    let mut three_of_a_kind = None;
    for (card, count) in freq {
        if count == 3 {
            three_of_a_kind = Some(card);
        }
    }

    if three_of_a_kind.is_some() && unique_cards.len() == 3 {
        return true;
    }

    false
}

#[allow(unused)]
fn is_two_pair(hand: &[Card]) -> bool {
    let mut freq: HashMap<&Card, u8> = HashMap::new();
    for card in hand {
        *freq.entry(card).or_insert(0) += 1;
    }

    let mut pair1 = None;
    let mut pair2 = None;
    for (card, count) in freq {
        if count == 2 && pair1.is_none() {
            pair1 = Some(card);
        } else if count == 2 {
            pair2 = Some(card);
        }
    }

    if pair1.is_some() && pair2.is_some() {
        return true;
    }

    false
}

#[allow(unused)]
fn is_one_pair(hand: &[Card]) -> bool {
    let unique_cards: HashSet<_> = hand.iter().collect();

    if unique_cards.len() == 4 {
        return true;
    }

    false
}

#[allow(unused)]
fn match_hand_to_ranking(hand: Vec<Card>) -> u8 {
    if is_five_of_a_kind(&hand) {
        return 6;
    }
    if is_four_of_a_kind(&hand) {
        return 5;
    }
    if is_full_house(&hand) {
        return 4;
    }
    if is_three_of_a_kind(&hand) {
        return 3;
    }
    if is_two_pair(&hand) {
        return 2;
    }
    if is_one_pair(&hand) {
        return 1;
    }

    0
}

// returns true if first hand wins
#[allow(unused)]
fn compare_high_card(hand1: Vec<Card>, hand2: Vec<Card>) -> bool {
    for i in 0..4 {
        let card1 = hand1.get(i).unwrap();
        let card2 = hand2.get(i).unwrap();

        if card1 > card2 {
            return true;
        }
        if card2 > card1 {
            return false;
        }

        // If same we are continuing with next card
        continue;
    }

    panic!("We must have found a decision earlier!");
}

fn match_char_to_card(char: &char) -> Card {
    if char.is_numeric() {
        let string_char = char.to_string().parse::<u8>().unwrap();

        let card: Card = match string_char {
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            _ => panic!("This value is not allowed as card value: {}", string_char),
        };

        return card;
    }

    let binding = char.to_string();
    let char_string = binding.as_str();

    let card: Card = match char_string {
        "A" => A,
        "K" => K,
        "Q" => Q,
        "J" => J,
        "T" => T,
        _ => panic!("This value is not allowed as card value: {}", char),
    };

    card
}

#[allow(unused)]
fn create_hand_from_string(line_string: &str) -> Vec<Card> {
    let mut return_hand: Vec<Card> = Default::default();
    let uppercase_sting = line_string.to_uppercase();

    if let Some(hand_string) = uppercase_sting.split(' ').next() {
        if line_string.len() != 5 {
            panic!("No hand given: {}", line_string)
        }

        for char in hand_string.chars() {
            return_hand.push(match_char_to_card(&char))
        }
    }

    return_hand
}

#[allow(dead_code)]
fn get_bid_from_string(line_string: &str) -> u16 {
    if let Some(bid_string) = line_string.split(' ').nth(1) {
        let bid: u16 = bid_string.parse::<u16>().unwrap();

        return bid;
    }

    panic!("No bid found: {}", line_string);
}

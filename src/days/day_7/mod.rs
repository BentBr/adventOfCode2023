use std::collections::{HashMap, HashSet};
use Card::{Eight, Five, Four, Nine, Seven, Six, Three, Two, A, J, K, Q, T};

mod first_question;
mod second_question;
mod tests;

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

    fn get_value_joker(&self) -> u8 {
        match self {
            A => 12,
            K => 11,
            Q => 10,
            T => 9,
            Nine => 8,
            Eight => 7,
            Seven => 6,
            Six => 5,
            Five => 4,
            Four => 3,
            Three => 2,
            Two => 1,
            J => 0,
        }
    }
}

pub fn solutions() {
    first_question::solution();
    second_question::solution();
}

fn is_five_of_a_kind(hand: &[Card], joker_style: bool) -> bool {
    let unique_cards: HashSet<_> = hand.iter().collect();

    if unique_cards.len() == 1 {
        return true;
    }
    false
}

fn is_four_of_a_kind(hand: &[Card], joker_style: bool) -> bool {
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

fn is_full_house(hand: &[Card], joker_style: bool) -> bool {
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

fn is_three_of_a_kind(hand: &[Card], joker_style: bool) -> bool {
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

fn is_two_pair(hand: &[Card], joker_style: bool) -> bool {
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

fn is_one_pair(hand: &[Card], joker_style: bool) -> bool {
    let unique_cards: HashSet<_> = hand.iter().collect();

    if unique_cards.len() == 4 {
        return true;
    }

    false
}

fn match_hand_to_ranking(hand: &[Card], joker_style: bool) -> u8 {
    if is_five_of_a_kind(hand, joker_style) {
        return 6;
    }
    if is_four_of_a_kind(hand, joker_style) {
        return 5;
    }
    if is_full_house(hand, joker_style) {
        return 4;
    }
    if is_three_of_a_kind(hand, joker_style) {
        return 3;
    }
    if is_two_pair(hand, joker_style) {
        return 2;
    }
    if is_one_pair(hand, joker_style) {
        return 1;
    }

    0
}

// returns true if first hand wins
fn compare_high_card(hand1: &[Card], hand2: &[Card], joker_style: bool) -> bool {
    for i in 0..5 {
        let card1: u8;
        let card2: u8;
        if joker_style {
            card1 = hand1.get(i).unwrap().get_value_joker();
            card2 = hand2.get(i).unwrap().get_value_joker();
        } else {
            card1 = hand1.get(i).unwrap().get_value();
            card2 = hand2.get(i).unwrap().get_value();
        }

        if card1 > card2 {
            return true;
        }
        if card2 > card1 {
            return false;
        }
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

fn create_hand_from_string(line_string: &str) -> Vec<Card> {
    let mut return_hand: Vec<Card> = Default::default();
    let uppercase_sting = line_string.to_uppercase();

    if let Some(hand_string) = uppercase_sting.split(' ').next() {
        if hand_string.len() != 5 {
            panic!("No hand given: {}", line_string)
        }

        for char in hand_string.chars() {
            return_hand.push(match_char_to_card(&char))
        }
    }

    return_hand
}

fn get_bid_from_string(line_string: &str) -> u16 {
    if let Some(bid_string) = line_string.split(' ').nth(1) {
        let bid: u16 = bid_string.parse::<u16>().unwrap();

        return bid;
    }

    panic!("No bid found: {}", line_string);
}

// Returns true if hand1 has higher ranking. Otherwise false.
fn compare_hand_ranking(hand1: &[Card], hand2: &[Card], joker_style: bool) -> bool {
    let ranking1: u8 = match_hand_to_ranking(hand1, joker_style);
    let ranking2: u8 = match_hand_to_ranking(hand2, joker_style);

    if ranking1 > ranking2 {
        return true;
    }
    if ranking1 < ranking2 {
        return false;
    }

    compare_high_card(hand1, hand2, joker_style)
}

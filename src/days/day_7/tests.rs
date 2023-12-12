#[cfg(test)]
mod tests {
    use crate::days::day_7::{
        compare_hand_ranking, compare_high_card, create_hand_from_string, get_bid_from_string,
        is_five_of_a_kind, is_four_of_a_kind, is_full_house, is_one_pair, is_three_of_a_kind,
        is_two_pair, match_char_to_card, match_hand_to_ranking, Card,
    };
    #[test]
    fn test_create_hand_from_string_numeric() {
        let string_positive = "23456";
        let hand_positive = create_hand_from_string(&string_positive);
        for (index, card) in hand_positive.iter().enumerate() {
            assert_eq!(usize::from(card.get_value()), index);
        }
    }

    #[test]
    #[should_panic]
    fn test_create_hand_from_string_numeric_fail() {
        let string_negative_length = "234567";
        let _hand_negative_length = create_hand_from_string(&string_negative_length);
    }

    #[test]
    fn test_create_hand_from_string_alpha() {
        let string_positive = "AQJKA";
        let hand_positive = create_hand_from_string(&string_positive);
        let mut result_int: u8 = 0;

        for card in hand_positive {
            result_int += card.get_value();
        }
        assert_eq!(result_int, 54);
    }

    #[test]
    #[should_panic]
    fn test_create_hand_from_string_alpha_fail() {
        let string_negative_length = "KQJKAA";
        let _hand_negative_length = create_hand_from_string(&string_negative_length);
    }

    #[test]
    fn test_create_hand_from_string_alpha_and_numeric() {
        let string_positive = "4Q2KA";
        let hand_positive = create_hand_from_string(&string_positive);
        let mut result_int: u8 = 0;

        for card in hand_positive {
            result_int += card.get_value();
        }
        assert_eq!(result_int, 35);
    }

    #[test]
    #[should_panic]
    fn test_create_hand_from_string_alpha_and_numeric_fail() {
        let string_negative_wrong_char = "B1JKA";
        let _hand_negative_wrong_char = create_hand_from_string(&string_negative_wrong_char);
    }

    #[test]
    fn test_create_hand_from_string_lowercase() {
        let string_positive = "akjk2";
        let hand_positive = create_hand_from_string(&string_positive);
        let mut result_int: u8 = 0;

        for card in hand_positive {
            result_int += card.get_value();
        }
        assert_eq!(result_int, 43);
    }

    #[test]
    #[should_panic]
    fn test_create_hand_from_string_lowercase_fail() {
        let string_negative_wrong_char = "akrk2";
        let _hand_negative_wrong_char = create_hand_from_string(&string_negative_wrong_char);
    }

    #[test]
    fn test_is_five_of_a_kind() {
        let string_positive = "AAAAA";
        let hand_positive = create_hand_from_string(&string_positive);
        let string_negative = "AAJAA";
        let hand_negative = create_hand_from_string(&string_negative);

        assert_eq!(is_five_of_a_kind(&hand_positive), true);
        assert_eq!(is_five_of_a_kind(&hand_negative), false);
    }

    #[test]
    fn test_is_four_of_a_kind() {
        let string_positive = "AAJAA";
        let hand_positive = create_hand_from_string(&string_positive);
        let string_negative = "AQJAA";
        let hand_negative = create_hand_from_string(&string_negative);

        assert_eq!(is_four_of_a_kind(&hand_positive), true);
        assert_eq!(is_four_of_a_kind(&hand_negative), false);
    }

    #[test]
    fn test_is_three_of_a_kind() {
        let string_positive = "QA5AA";
        let hand_positive = create_hand_from_string(&string_positive);
        let string_negative = "2QJAA";
        let hand_negative = create_hand_from_string(&string_negative);
        let string_negative_four = "2AAAA";
        let hand_negative_four = create_hand_from_string(&string_negative_four);

        assert_eq!(is_three_of_a_kind(&hand_positive), true);
        assert_eq!(is_three_of_a_kind(&hand_negative), false);
        assert_eq!(is_three_of_a_kind(&hand_negative_four), false);
    }

    #[test]
    fn test_is_two_pair() {
        let string_positive = "AAJQQ";
        let hand_positive = create_hand_from_string(&string_positive);
        let string_negative_three = "AQQAA";
        let hand_negative_three = create_hand_from_string(&string_negative_three);
        let string_negative_same = "AAJAA";
        let hand_negative_same = create_hand_from_string(&string_negative_same);

        assert_eq!(is_two_pair(&hand_positive), true);
        assert_eq!(is_two_pair(&hand_negative_three), false);
        assert_eq!(is_two_pair(&hand_negative_same), false);
    }
    #[test]
    fn test_is_one_pair() {
        let string_positive = "AAJQ3";
        let hand_positive = create_hand_from_string(&string_positive);
        let string_negative = "AQJAA";
        let hand_negative = create_hand_from_string(&string_negative);

        assert_eq!(is_one_pair(&hand_positive), true);
        assert_eq!(is_one_pair(&hand_negative), false);
    }

    #[test]
    fn test_is_full_house() {
        let string_positive = "AAJJJ";
        let hand_positive = create_hand_from_string(&string_positive);
        let string_negative = "AQJAA";
        let hand_negative = create_hand_from_string(&string_negative);

        assert_eq!(is_full_house(&hand_positive), true);
        assert_eq!(is_full_house(&hand_negative), false);
    }

    #[test]
    fn test_high_card() {
        let high1 = "23456";
        let hand_high1 = create_hand_from_string(&high1);
        let high2 = "34567";
        let hand_high2 = create_hand_from_string(&high2);

        assert!(compare_high_card(&hand_high2, &hand_high1));
    }

    #[test]
    fn test_get_bid_from_string() {
        let string_one: &str = "KK677 28";
        let string_two: &str = "KTJJT 220";

        assert_eq!(get_bid_from_string(string_one), 28);
        assert_eq!(get_bid_from_string(string_two), 220);
    }

    #[test]
    #[should_panic]
    fn test_get_bid_from_string_fail() {
        let string_one: &str = "KK677";

        assert_eq!(get_bid_from_string(string_one), 28);
    }

    #[test]
    fn test_match_char_to_card() {
        let char: char = "A".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::A);

        let char: char = "K".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::K);

        let char: char = "Q".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Q);

        let char: char = "J".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::J);

        let char: char = "T".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::T);

        let char: char = "2".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Two);

        let char: char = "3".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Three);

        let char: char = "4".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Four);

        let char: char = "5".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Five);

        let char: char = "6".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Six);

        let char: char = "7".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Seven);

        let char: char = "8".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Eight);

        let char: char = "9".chars().next().unwrap();
        assert_eq!(match_char_to_card(&char), Card::Nine);
    }

    #[test]
    fn test_ranking_five_four() {
        let hand1 = "AAAAA";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "AAAAT";
        let hand2 = create_hand_from_string(&hand2);

        assert!(match_hand_to_ranking(&hand1) > match_hand_to_ranking(&hand2))
    }

    #[test]
    fn test_ranking_four_three() {
        let hand1 = "AATAA";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "AAQAT";
        let hand2 = create_hand_from_string(&hand2);

        assert!(match_hand_to_ranking(&hand1) > match_hand_to_ranking(&hand2))
    }

    #[test]
    fn test_ranking_full_three() {
        let hand1 = "QQAAA";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "AAA4T";
        let hand2 = create_hand_from_string(&hand2);

        assert!(match_hand_to_ranking(&hand1) > match_hand_to_ranking(&hand2))
    }

    #[test]
    fn test_ranking_two_pair_three() {
        let hand1 = "AAQQ4";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "AAA6T";
        let hand2 = create_hand_from_string(&hand2);

        assert!(match_hand_to_ranking(&hand1) < match_hand_to_ranking(&hand2))
    }

    #[test]
    fn test_ranking_two_pair_one_pair() {
        let hand1 = "AAQQT";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "AA45T";
        let hand2 = create_hand_from_string(&hand2);

        assert!(match_hand_to_ranking(&hand1) > match_hand_to_ranking(&hand2))
    }

    #[test]
    fn test_compare_hand_ranking() {
        // 2 pair vs 1 pair
        let hand1 = "AAQQT";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "AA45T";
        let hand2 = create_hand_from_string(&hand2);

        assert!(compare_hand_ranking(&hand1, &hand2));

        // Five vs 1 pair
        let hand1 = "AAAAA";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "AA45T";
        let hand2 = create_hand_from_string(&hand2);

        assert!(compare_hand_ranking(&hand1, &hand2));

        // High card vs high card
        let hand1 = "34567";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "24567";
        let hand2 = create_hand_from_string(&hand2);

        assert!(compare_hand_ranking(&hand1, &hand2));

        // 1 pair vs full house
        let hand1 = "AQKTT";
        let hand1 = create_hand_from_string(&hand1);
        let hand2 = "QQQ22";
        let hand2 = create_hand_from_string(&hand2);

        assert!(!compare_hand_ranking(&hand1, &hand2));
    }
}

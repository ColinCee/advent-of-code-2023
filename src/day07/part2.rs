// Example
// 32T3K 765  --> 1
// T55J5 684  --> 4
// KK677 28  --> 3
// KTJJT 220  --> 2
// QQQJA 483 --> 5

use std::{cmp::Ordering, collections::HashMap};

use crate::utils::read_file_lines::read_file_lines;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
}
pub fn run() {
    let lines = read_file_lines("./src/day07/inputData.txt");
    let mut cards = Vec::new();

    for line in lines.iter() {
        let mut card = line.split_whitespace();
        let card_number = card.next().unwrap();
        let card_value = card.next().unwrap().parse::<i32>().unwrap();
        cards.push(Hand {
            cards: card_number.to_string(),
            bid: card_value,
        });
    }

    cards.sort_by(card_power_sort);
    println!("{:?}", cards);

    let mut total = 0;
    for i in 0..cards.len() {
        total += cards[i].bid * (i + 1) as i32;
    }

    println!("Total: {}", total);
}

fn card_power_sort(a: &Hand, b: &Hand) -> Ordering {
    let a_rank = get_hand_ranking(a);
    let b_rank = get_hand_ranking(b);
    println!("a: {:?} a_rank: {} b: {:?}b_rank: {}", a, a_rank, b, b_rank);
    if a_rank == b_rank {
        // if both hands are the same rank, sort by highest individual card ranks left to right
        let a_cards = a.cards.chars().collect::<Vec<char>>();
        let b_cards = b.cards.chars().collect::<Vec<char>>();

        for i in 0..a_cards.len() {
            let a_card = get_value_of_card(a_cards[i]);
            let b_card = get_value_of_card(b_cards[i]);

            if a_card != b_card {
                return a_card.cmp(&b_card);
            }
        }
    }

    a_rank.cmp(&b_rank)
}
fn get_value_of_card(card: char) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1, // special wildcard rule
        'T' => 10,
        _ => card.to_digit(10).unwrap() as i32,
    }
}

fn get_hand_ranking(hand: &Hand) -> u8 {
    let char_map = hand.cards.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    if char_map.contains_key(&'J') {
        let num_jacks = char_map.get(&'J').unwrap();
        if num_jacks == &5 {
            return 6; // five of a kind
        }
        let max_other_card_count = char_map
            .iter()
            .filter(|(&k, _)| k != 'J')
            .map(|(_, &v)| v)
            .max()
            .unwrap();

        if num_jacks == &4 {
            return 6; // five of a kind
        }

        if num_jacks == &3 {
            if max_other_card_count == 2 {
                return 6; // Five of a kind
            }

            return 5; // Four of a kind
        }

        if num_jacks == &2 {
            if max_other_card_count == 3 {
                return 6; // Five of a kind
            }

            if max_other_card_count == 2 {
                return 5; // Four of a kind
            }

            return 3; // Three of a kind
        }

        // num_jacks == 1
        if max_other_card_count == 4 {
            return 6; // Five of a kind
        }

        if max_other_card_count == 3 {
            return 5; // Four of a kind
        }

        if max_other_card_count == 2 {
            // check if there is another pair
            let pair_count = char_map.iter().filter(|(_, &v)| v == 2).count();

            if pair_count == 2 {
                return 4; // Full house
            }

            return 3; // Three of a kind
        }

        return 1;
    }

    match char_map.len() {
        1 => 6, // five of a kind
        2 => {
            if char_map.values().any(|&v| v == 4) {
                5 // four of a kind
            } else {
                4 // full house
            }
        }
        3 => {
            if char_map.values().any(|&v| v == 3) {
                3 // three of a kind
            } else {
                2 // two pair
            }
        }
        4 => 1, // one pair
        _ => 0, // high card
    }
}

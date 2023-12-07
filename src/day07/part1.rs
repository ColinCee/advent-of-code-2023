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
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as i32,
    }
}
fn get_hand_ranking(hand: &Hand) -> u8 {
    // five of a kind
    // four of a kind
    // full house
    // three of a kind
    // two pair
    // one pair
    // high card

    let char_map = hand.cards.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    // five of a kind
    if char_map.len() == 1 {
        return 6;
    }

    // either full house or four of a kind
    if char_map.len() == 2 {
        if char_map.values().any(|&v| v == 4) {
            return 5;
        }

        return 4;
    }

    // either three of a kind or two pair
    if char_map.len() == 3 {
        if char_map.values().any(|&v| v == 3) {
            return 3;
        }

        return 2;
    }

    if char_map.len() == 4 {
        return 1;
    }

    0
}

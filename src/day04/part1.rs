// Example:
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

use std::{collections::HashSet, fs::read_to_string};

pub fn run() {
    // read file
    let input = read_to_string("./src/day04/inputData.txt").unwrap();

    // for each line, split on the colon
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        let card_numbers = read_card_numbers(line).unwrap();
        let winning_numbers = read_winning_numbers(line).unwrap();

        let score = card_numbers
            .iter()
            .filter(|num| winning_numbers.contains(num))
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 });

        println!(
            "Card Nums: {:?} Winning {:?} Score: {}",
            card_numbers, winning_numbers, score
        );

        total += score;
    }

    println!("Total: {}", total);
}

fn read_card_numbers(line: &str) -> Result<HashSet<i32>, &'static str> {
    let (card_number_section, _) = line.split_once('|').ok_or("No pipe symbol found")?;
    let (_, card_numbers) = card_number_section
        .split_once(':')
        .ok_or("No colon symbol found")?;

    card_numbers
        .split_whitespace()
        .map(|num| num.parse::<i32>())
        .collect::<Result<HashSet<_>, _>>()
        .map_err(|_| "Failed to parse number")
}

fn read_winning_numbers(line: &str) -> Result<HashSet<i32>, &'static str> {
    line.split_once('|')
        .ok_or("No pipe symbol found")?
        .1
        .split_whitespace()
        .map(|num| num.parse::<i32>())
        .collect::<Result<HashSet<_>, _>>()
        .map_err(|_| "Failed to parse number")
}

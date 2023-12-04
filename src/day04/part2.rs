use std::{collections::HashSet, fs::read_to_string};

pub fn run() {
    // read file
    let input = read_to_string("./src/day04/inputData.txt").unwrap();

    // for each line, split on the colon
    let lines: Vec<&str> = input.lines().collect();

    let mut copies_counts = vec![0 as i32; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        copies_counts[index] += 1;
        let num_of_winning_numbers = get_num_of_winning_numbers(line);

        for i in 0..num_of_winning_numbers {
            copies_counts[index + 1 + i as usize] += copies_counts[index];
        }

        println!("Copies counts: {:?}", copies_counts);
    }

    println!("Total: {}", copies_counts.iter().sum::<i32>());
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

fn get_num_of_winning_numbers(line: &str) -> i32 {
    let card_numbers = read_card_numbers(line).unwrap();
    let winning_numbers = read_winning_numbers(line).unwrap();

    card_numbers
        .iter()
        .filter(|num| winning_numbers.contains(num))
        .count() as i32
}

use std::{collections::{HashMap, HashSet}, fs};

pub fn run() {
    let contents = fs::read_to_string("./src/day01/part1.txt")
        .expect("Something went wrong reading the file");

    let mut total = 0;
    for line in contents.lines() {
        let first_num = find_first_number(line);
        let last_num = find_last_number(line);
        println!("First number: {:?}, Last number: {:?}", first_num, last_num);
        
        let concat_num = format!("{}{}", first_num.unwrap(), last_num.unwrap()).parse::<i32>().unwrap();
        total += concat_num;
    }

    println!("Total: {}", total);
    // println!("First line: {:?}", first_line);
    // let num = find_first_number(first_line);
    // println!("First number: {:?}", num);

    // let last_num = find_last_number(first_line);
    // println!("Last number: {:?}", last_num);
}

pub fn get_words_map() -> HashMap<&'static str, i32> {
    let number_words = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
    ].iter().cloned().collect::<HashMap<_, _>>();

    number_words
}

// given an index, return all the possible letters at that index
pub fn get_possible_letters_at_index(index: i32) -> HashSet<String> {
    let words_map = get_words_map();
    let mut possible_letters = HashSet::new();

    for (word, number) in words_map {
        let letter = word.chars().nth(index as usize);
        if letter.is_none()  {
            continue;
        }
        possible_letters.insert(letter.unwrap().to_string());
    }

    possible_letters
}

fn get_reverse_possible_letters_at_index(index: i32) -> HashSet<String> {
    let words_map = get_words_map();
    let mut possible_letters = HashSet::new();

    for (word, number) in words_map {
        let letter = word.chars().rev().nth(index as usize);
        if letter.is_none()  {
            continue;
        }
        possible_letters.insert(letter.unwrap().to_string());
    }

    possible_letters
}


fn find_first_number(s: &str) -> Option<i32> {

    let words_map = get_words_map();
    let mut word = String::new();

    for (i, char) in s.chars().enumerate() {
        if char.is_digit(10) {
            return Some(char.to_digit(10).unwrap() as i32);
        }

        let possible_letters = get_possible_letters_at_index(word.len() as i32);
        // println!("Current letter {}, Possible letters: {:?}, current word {:?}", char, possible_letters, word);
        if possible_letters.contains(&char.to_string()) {
            word.push(char);
        } else {
            word.clear();
        }

        // if word in words_map 
        if words_map.contains_key(word.as_str()) {
            return Some(*words_map.get(word.as_str()).unwrap());
        }
    }

    None
}

fn find_last_number(s: &str) -> Option<i32> {
    let words_map = get_words_map();
    let mut word = Vec::new();

    for (i, char) in s.chars().rev().enumerate() {
        if char.is_digit(10) {
            return Some(char.to_digit(10).unwrap() as i32);
        }

        let possible_letters = get_reverse_possible_letters_at_index(word.len() as i32);
        // println!("Current letter {}, Possible letters: {:?}, current word {:?}", char, possible_letters, word);
        if possible_letters.contains(&char.to_string()) {
            word.push(char);
        } else {
            word.clear();
        }

        // if word in words_map 
        let current_word = word.iter().rev().collect::<String>();
        // println!("Current word: {:?}", current_word);
        if words_map.contains_key(current_word.as_str()) {
            return Some(*words_map.get(&current_word.as_str()).unwrap());
        }
    }

    None
}

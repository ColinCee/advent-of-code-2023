use std::{collections::{HashMap, HashSet}, fs};
use crate::day01::trie::Trie;

pub fn run() {
    let contents = fs::read_to_string("./src/day01/part1.txt")
        .expect("Something went wrong reading the file");

    let trie = build_trie();
    let reverse_trie = build_reverse_trie();
    // debug reverse_trie
    // print each child starting with e
    reverse_trie.print_children_starting_with("e");


    let mut total = 0;
    for line in contents.lines() {
        let first_num = find_first_number(line, &trie);
        let last_num = find_last_number(line, &reverse_trie);
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

fn build_trie() -> Trie {
    let mut trie = Trie::new();
    let words = get_words_map();
    for (word, _) in words {
        trie.insert(word);
    }
    trie
}

fn build_reverse_trie() -> Trie {
    let mut trie = Trie::new();
    let words = get_words_map();
    for (word, _) in words {
        let reverse_word = word.chars().rev().collect::<String>();
        trie.insert(&reverse_word);
    }
    trie
}
pub fn get_words_map() -> HashMap<&'static str, i32> {
    let number_words = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
    ].iter().cloned().collect::<HashMap<_, _>>();

    number_words
}

fn find_first_number(s: &str, trie: &Trie) -> Option<i32> {

    let words_map = get_words_map();
    let mut word = Vec::new();

    for (i, char) in s.chars().enumerate() {
        if char.is_digit(10) {
            return Some(char.to_digit(10).unwrap() as i32);
        }

        let mut current_word = word.iter().collect::<String>();
        let new_word = format!("{}{}", current_word, char);
        let word_exists = trie.starts_with(&new_word);
        println!("current_word: {}, new_word: {}, word_exists: {}", current_word, new_word, word_exists);
        if !word_exists {
            word.clear();
            word.push(char);
            continue;
        }

        word.push(char);
        current_word = word.iter().collect::<String>();
        if words_map.contains_key(current_word.as_str()) {
            return Some(*words_map.get(current_word.as_str()).unwrap());
        }
    }

    None
}

fn find_last_number(s: &str, trie: &Trie) -> Option<i32> {
    let words_map = get_words_map();
    let mut word: Vec<char> = Vec::new();

    for (i, char) in s.chars().rev().enumerate() {
        if char.is_digit(10) {
            return Some(char.to_digit(10).unwrap() as i32);
        }

        let mut current_word = word.iter().collect::<String>();
        let new_word = format!("{}{}", current_word, char);
        let word_exists = trie.starts_with(&new_word);
        println!("char: {} current_word: {}, new_word: {}, word_exists: {}", char, current_word, new_word, word_exists);

        if !word_exists {
            word.clear();
            word.push(char);
            continue;
        }

        word.push(char);
        current_word = word.iter().rev().collect::<String>();
        
        if words_map.contains_key(current_word.as_str()) {
            return Some(*words_map.get(current_word.as_str()).unwrap());
        }
    }

    None
}

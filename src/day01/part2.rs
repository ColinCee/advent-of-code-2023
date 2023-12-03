use std::{collections::{HashMap, HashSet}, fs};
use crate::day01::trie::Trie;

pub fn run() {
    let contents = fs::read_to_string("./src/day01/part1.txt")
        .expect("Something went wrong reading the file");

    // // test contents bvdneightsevenfcjnhccrlb7nine
    // let contents = "5vsrcnine";
    let trie = build_trie();
    let reverse_trie = build_reverse_trie();

    let mut total = 0;
    for line in contents.lines() {
        let first_num = find_first_number(line, &trie);
        let last_num = find_last_number(line, &reverse_trie);
        println!("line: {} First number: {:?}, Last number: {:?}",line, first_num, last_num);
        
        let concat_num = format!("{}{}", first_num.unwrap(), last_num.unwrap()).parse::<i32>().unwrap();
        total += concat_num;
    }

    println!("Total: {}", total);
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
    for (i, char) in s.chars().enumerate() {
        if char.is_digit(10) {
            return Some(char.to_digit(10).unwrap() as i32);
        }

        let substring_number = find_string_number_in_str(&s[i..], trie);
        println!("substring_number: {:?}", substring_number);
        if substring_number.is_some() {
            return substring_number;
        }
    }

    None
}

fn find_string_number_in_str(s: &str, trie: &Trie) -> Option<i32> {
    let words_map = get_words_map();

    let mut word = String::new();
    let mut node_exists = trie.children.get(&s.chars().nth(0).unwrap());

    let mut i = 0;
    while node_exists.is_some() {    
        println!("currentChar: {} node_exists: {:?}", s.chars().nth(i).unwrap(), node_exists);
        word.push(s.chars().nth(i).unwrap());
        if node_exists.unwrap().end_of_word {
            let number = words_map.get(&word[..]);
            if number.is_some() {
                return number.cloned();
            }
        }

        i += 1;
        node_exists = node_exists.unwrap().children.get(&s.chars().nth(i).unwrap());

    }

    None
}

fn find_last_number(s: &str, trie: &Trie) -> Option<i32> {
    let reversed_s = s.chars().rev().collect::<String>();
    for (i, char) in s.chars().rev().enumerate() {
        if char.is_digit(10) {
            return Some(char.to_digit(10).unwrap() as i32);
        }

        let substring_number = find_last_string_number_in_str(&reversed_s[i..], trie);
        // println!("substring_number: {:?}", substring_number);
        if substring_number.is_some() {
            return substring_number;
        }
    }

    None
}

fn find_last_string_number_in_str(s: &str, trie: &Trie) -> Option<i32> {
    let words_map = get_words_map();

    let mut word = String::new();
    let mut node_exists = trie.children.get(&s.chars().nth(0).unwrap());

    let mut i = 0;
    while node_exists.is_some() {    
        println!("currentChar: {} node_exists: {:?}", s.chars().nth(i).unwrap(), node_exists);
        word.push(s.chars().nth(i).unwrap());
        if node_exists.unwrap().end_of_word {
            let reversed_word = word.chars().rev().collect::<String>();
            let number = words_map.get(&reversed_word[..]);
            if number.is_some() {
                return number.cloned();
            }
        }

        i += 1;
        node_exists = node_exists.unwrap().children.get(&s.chars().nth(i).unwrap());

    }

    None
}

use core::num;
use std::collections::{VecDeque, HashMap};

pub fn run() {
    let input = std::fs::read_to_string("./src/day03/inputData.txt").unwrap();
    let input_2d_vec = create_input_2d_vec(input.clone());

    let mut gear_map = HashMap::new();

    for (i, row) in input_2d_vec.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &'*' {
                let nums = get_adjacent_numbers(input_2d_vec.clone(), i, j);
                gear_map.insert((i, j), nums);
            }
        }
    }
    println!("{:?}", gear_map);
    // filter only the ones that have 2 adjacent numbers
    let valid = gear_map.iter().filter(|(_, nums)| nums.len() == 2).collect::<Vec<_>>();
    println!("{:?}", valid);
    // multiply each of the two nums and sum total
    let sum: i32 = valid.iter().map(|(_, nums)| nums.iter().product::<i32>()).sum();
    println!("Sum: {}", sum);
}

fn create_input_2d_vec(input: String) -> Vec<Vec<char>> {
    let lines = input.lines();
    let num_rows = lines.clone().count();
    let num_cols = lines.clone().nth(0).unwrap().chars().count();
    let mut input_2d_vec: Vec<Vec<char>> = vec![vec![' '; num_cols]; num_rows];

    for (i, row) in lines.enumerate() {
        // size should be the same for all rows
        println!("{}", i);
        for (j, char) in row.chars().enumerate() {
            
            input_2d_vec[i][j] = char;
        }
    }

    input_2d_vec
}

fn get_adjacent_numbers(input: Vec<Vec<char>>, i: usize, j: usize) -> Vec<i32> {
    let mut adjacent_numbers: Vec<i32> = Vec::new();

    // For left and right we just need to prepend/append each digit until no more digits

    let mut num_string: VecDeque<char> = VecDeque::new();
    // left
    let mut k = j;
    while k > 0 && input[i][k-1].is_digit(10) {
        num_string.push_front(input[i][k-1]);
        k -= 1;
    }
    if num_string.len() > 0 {
        adjacent_numbers.push(num_string.iter().collect::<String>().parse::<i32>().unwrap());
    }

    // right
    num_string.clear();
    let mut k = j;
    while k < input[i].len() - 1 && input[i][k+1].is_digit(10) {
        num_string.push_back(input[i][k+1]);
        k += 1;
    }
    if num_string.len() > 0 {
        adjacent_numbers.push(num_string.iter().collect::<String>().parse::<i32>().unwrap());
    }
    // up left
    if i > 0 {
        num_string.clear();
        let mut k = j;

        while k > 0 && input[i-1][k-1].is_digit(10) {
            num_string.push_front(input[i-1][k-1]);
            k -= 1;
        }
        let mut k = j - 1;
        while k < input[i-1].len() - 1 && input[i-1][k+1].is_digit(10) {
            num_string.push_back(input[i-1][k+1]);
            k += 1;
        }

        if num_string.len() > 0 {
            adjacent_numbers.push(num_string.iter().collect::<String>().parse::<i32>().unwrap());
        }

        // when we scanned left to right the last digit was at the top left, meaning the top-middle was empty
        // we can now just start at the top right and scan right
        if k == j - 1 {
            num_string.clear();
            let mut k = j;
            while k < input[i-1].len() - 1 && input[i-1][k+1].is_digit(10) {
                num_string.push_back(input[i-1][k+1]);
                k += 1;
            }
            if num_string.len() > 0 {
                adjacent_numbers.push(num_string.iter().collect::<String>().parse::<i32>().unwrap());
            }
        }

    }

    // down
    if i < input.len() - 1{
        num_string.clear();
        let mut k = j;

        while k > 0 && input[i+1][k-1].is_digit(10) {
            num_string.push_front(input[i+1][k-1]);
            k -= 1;
        }
        let mut k = j - 1;
        while k < input[i+1].len() - 1 && input[i+1][k+1].is_digit(10) {
            num_string.push_back(input[i+1][k+1]);
            k += 1;
        }

        if num_string.len() > 0 {
            adjacent_numbers.push(num_string.iter().collect::<String>().parse::<i32>().unwrap());
        }

        // when we scanned left to right the last digit was at the bottom left, meaning the bottom-middle was empty
        // we can now just start at the bottom right and scan right
        if k == j - 1 {
            num_string.clear();
            let mut k = j;
            while k < input[i+1].len() - 1 && input[i+1][k+1].is_digit(10) {
                num_string.push_back(input[i+1][k+1]);
                k += 1;
            }
            if num_string.len() > 0 {
                adjacent_numbers.push(num_string.iter().collect::<String>().parse::<i32>().unwrap());
            }
        }
    }
    adjacent_numbers
}

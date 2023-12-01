use std::fs;

pub fn run() {
    let contents = fs::read_to_string("./src/day01/part1.txt")
        .expect("Something went wrong reading the file");


    // for each line
        // use a two pointer approach
        // left pointer starts at 0
        // right pointer starts at end of string
        // left pointer find the first number moving right
        // right pointer find the first number moving left
        // concatenate two numbers together
        // add to total
    
    let mut total = 0;

    for line in contents.lines() {
        let left_num = get_left_number(line);
        let right_number = get_right_number(line);

        let concat_num = format!("{}{}", left_num, right_number).parse::<i32>().unwrap();
        total += concat_num;
    }
    
    println!("Total: {}", total);
}

fn get_left_number(line: &str) -> i32 {
    let mut left = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            left = c.to_digit(10).unwrap() as i32;
            break;
        }
    }
    left
}

fn get_right_number(line: &str) -> i32 {
    let mut right = 0;
    for c in line.chars().rev() {
        if c.is_digit(10) {
            right = c.to_digit(10).unwrap() as i32;
            break;
        }
    }
    right
}

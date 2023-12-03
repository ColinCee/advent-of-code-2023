use std::collections::VecDeque;

pub fn run() {
    let input = std::fs::read_to_string("./src/day03/inputData.txt").unwrap();
    let input_2d_vec = create_input_2d_vec(input.clone());
    let mut mask = create_symbol_mask(input);
    // pretty print the mask
    for row in &mask {
        for c in row {
            if *c {
                print!(" X ");
            } else {
                print!(" . ");
            }
        }
        println!();
    }

    let valid_nums = find_valid_nums(input_2d_vec, &mut mask);
    println!("{:?}", valid_nums);
    let sum: i32 = valid_nums.iter().sum();
    println!("Sum: {}", sum);
}

fn find_valid_nums(input: Vec<Vec<char>>, mask: &mut Vec<Vec<bool>>) -> Vec<i32> {
    let mut valid_nums: Vec<i32> = Vec::new();

    for (i, row) in input.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if !mask[i][j] {
                continue
            }

            if !input[i][j].is_digit(10) {
                continue
            }

            let mut num_string: VecDeque<char> = VecDeque::new();
            num_string.push_front(input[i][j]);
            mask[i][j] = false;

            // iterate left until we hit a . or end of row
            let mut k = j;
            while k > 0 && input[i][k-1].is_digit(10) {
                num_string.push_front(input[i][k-1]);
                k -= 1;
                mask[i][k] = false;
            }

            // iterate right until we hit a . or end of row
            let mut k = j;
            while k < input[i].len() - 1 && input[i][k+1].is_digit(10) {
                num_string.push_back(input[i][k+1]);
                k += 1;
                mask[i][k] = false;
            }

            let num = num_string.iter().collect::<String>().parse::<i32>().unwrap();
            valid_nums.push(num);
        }
    }

    valid_nums
}

fn create_input_2d_vec(input: String) -> Vec<Vec<char>> {
    let lines = input.lines();
    let num_rows = lines.clone().count();
    let num_cols = lines.clone().nth(0).unwrap().chars().count();
    let mut input_2d_vec: Vec<Vec<char>> = vec![vec![' '; num_cols]; num_rows];

    for (i, row) in lines.enumerate() {
        // size should be the same for all rows

        for (j, char) in row.chars().enumerate() {
            input_2d_vec[i][j] = char;
        }
    }

    input_2d_vec
}

fn create_symbol_mask(input: String) -> Vec<Vec<bool>> {
    let lines = input.lines();
    let num_rows = lines.clone().count();
    let num_cols = lines.clone().nth(0).unwrap().chars().count();
    let mut mask: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];

    for (i, row) in lines.enumerate() {
        // size should be the same for all rows

        for (j, char) in row.chars().enumerate() {
            // if not digit or . then it's a symbol
            if !char.is_digit(10) && char != '.' {
                // in all cardinal directors of i,j, set to true
                set_true_to_adjacent_cells(&mut mask, i, j);
            }
        }
    }

    mask
}

// including diagonals
fn set_true_to_adjacent_cells(mask: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    // set the cell at i-1,j to true
    if i > 0 {
        mask[i-1][j] = true;
    }

    // set the cell at i+1,j to true
    if i < mask.len() - 1 {
        mask[i+1][j] = true;
    }

    // set the cell at i,j-1 to true
    if j > 0 {
        mask[i][j-1] = true;
    }

    // set the cell at i,j+1 to true
    if j < mask[i].len() - 1 {
        mask[i][j+1] = true;
    }

    // set the cell at i-1,j-1 to true
    if i > 0 && j > 0 {
        mask[i-1][j-1] = true;
    }

    // set the cell at i-1,j+1 to true
    if i > 0 && j < mask[i].len() - 1 {
        mask[i-1][j+1] = true;
    }

    // set the cell at i+1,j-1 to true
    if i < mask.len() - 1 && j > 0 {
        mask[i+1][j-1] = true;
    }

    // set the cell at i+1,j+1 to true
    if i < mask.len() - 1 && j < mask[i].len() - 1 {
        mask[i+1][j+1] = true;
    }
}

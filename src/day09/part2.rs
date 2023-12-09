use crate::utils::read_file_lines::read_file_lines;

fn parse_histories(lines: &Vec<String>) -> Vec<Vec<i64>> {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}
fn get_diff_vectors(history: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut diff_vectors: Vec<Vec<i64>> = Vec::new();
    diff_vectors.push(history.clone());

    while diff_vectors.last().unwrap().iter().any(|num| num != &0) {
        let current = diff_vectors.last().unwrap();

        let next = current
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<i64>>();

        diff_vectors.push(next);
    }

    diff_vectors
}

fn find_next_history_value(diff_vectors: &Vec<Vec<i64>>) -> i64 {
    diff_vectors.windows(2).rev().fold(0, |acc, window| {
        let first_next = window[0].first().unwrap();
        println!("first_current: {:?} first_current {}", acc, first_next);
        first_next - acc
    })
}

pub fn run() {
    let lines = read_file_lines("src/day09/inputData.txt");

    let histories: Vec<Vec<i64>> = parse_histories(&lines);
    println!("histories: {:?}", histories);

    let sum_next_histories = histories
        .iter()
        .map(|history| {
            let diff_vector = get_diff_vectors(history);

            println!("diff_vector: {:?}", diff_vector);

            let next_history_value = find_next_history_value(&diff_vector);
            println!("next_history_value: {:?}", next_history_value);

            next_history_value
        })
        .sum::<i64>();

    println!("sum_next_histories: {:?}", sum_next_histories);
}

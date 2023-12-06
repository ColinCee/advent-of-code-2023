use crate::utils::read_file::read_file;
use std::collections::HashSet;

pub fn run() {
    let lines = read_file("./src/day05/inputData.txt");

    let seeds = get_seeds_set(&lines[0]);

    let mut index = 1;
    let mut locations: HashSet<i64> = HashSet::new();

    for seed in seeds {
        println!("seed {}", seed);
        let mut source = seed;
        let mut dest = -1;
        while index < lines.len() {
            let line = &lines[index];
            if line.contains("map") {
                index += 1;
                dest = find_dest_num(&mut index, &lines, &source);
                println!("source {} dest {}", source, dest);
                source = dest;
            }

            index += 1;
        }

        locations.insert(dest);
        index = 1;
    }

    // print min location number
    println!("min location number {}", locations.iter().min().unwrap());
}

fn find_dest_num(index: &mut usize, lines: &Vec<String>, source_num: &i64) -> i64 {
    while index < &mut lines.len() && !lines[*index].is_empty() {
        let line = &lines[*index];

        let map_line = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i64>().ok())
            .collect::<Vec<i64>>();

        if map_line[1] <= source_num.to_owned()
            && source_num.to_owned() <= map_line[1] + map_line[2] - 1
        {
            let diff = map_line[0] - map_line[1];
            return source_num + diff;
        }

        *index += 1;
    }

    return source_num.to_owned();
}
fn get_seeds_set(line: &str) -> HashSet<i64> {
    line.split_whitespace()
        .skip(1)
        .filter_map(|num| num.parse::<i64>().ok())
        .collect()
}

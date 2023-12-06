use crate::utils::read_file_lines::read_file_lines;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    vec,
};

#[derive(Debug)]
struct SourceRange {
    start: u64,
    end: u64,
    name: String,
}

#[derive(Debug)]
struct MapRange {
    source_start: u64,
    source_end: u64,
    dest_start: u64,
    dest_end: u64,
    name: String,
}

fn get_seeds(line: &str) -> Vec<SourceRange> {
    line.split_whitespace()
        .skip(1) // Skip the "seeds:" part
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>() // Collect numbers into a Vec
        .chunks(2) // Iterate over the numbers in pairs
        .filter_map(|pair| {
            if pair.len() == 2 {
                Some(SourceRange {
                    start: pair[0],
                    end: pair[0] + pair[1] - 1,
                    name: "seed".to_string(),
                })
            } else {
                None
            }
        })
        .collect() // Collect Seed structs into a Vec
}

fn read_maps(lines: &[String]) -> HashMap<&str, Vec<MapRange>> {
    let mut maps = HashMap::new();

    let mut index = 0;
    let mut current_map_name = "";

    while index < lines.len() - 1 {
        let line = lines[index].as_str();
        index += 1;

        if line.contains("map") {
            current_map_name = line;
            maps.insert(current_map_name, Vec::new());
            continue;
        }

        if line.is_empty() || current_map_name.is_empty() {
            continue;
        }

        let numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        assert_eq!(numbers.len(), 3, "Expected 3 numbers per line");

        maps.get_mut(current_map_name).unwrap().push(MapRange {
            source_start: numbers[1],
            source_end: numbers[1] + numbers[2] - 1,
            dest_start: numbers[0],
            dest_end: numbers[0] + numbers[2] - 1,
            name: current_map_name.to_string(),
        });
    }

    maps
}

pub fn run() {
    let lines = read_file_lines("./src/day05/testData.txt");
    let seeds = get_seeds(&lines[0]);
    println!("seeds: {:?}", seeds);

    let maps = read_maps(&lines[1..]);
    for (name, map) in maps.iter() {
        println!("------- {} ------", name);
        for range in map {
            println!("{:?} ", range);
        }
    }
}

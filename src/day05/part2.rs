use crate::utils::read_file_lines::read_file_lines;
use std::{
    borrow::Cow,
    cmp::{max, min},
    collections::{HashMap, HashSet},
    vec,
};
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SourceRange {
    start: i64,
    end: i64,
    name: String,
}

#[derive(Debug)]
struct MapRange {
    source_start: i64,
    source_end: i64,
    dest_start: i64,
    dest_end: i64,
    name: String,
}

fn get_seeds(line: &str) -> Vec<SourceRange> {
    line.split_whitespace()
        .skip(1) // Skip the "seeds:" part
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>() // Collect numbers into a Vec
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
            current_map_name = line.split_whitespace().next().unwrap();
            assert!(!current_map_name.contains("map-"));
            maps.insert(current_map_name, Vec::new());
            continue;
        }

        if line.is_empty() || current_map_name.is_empty() {
            continue;
        }

        let numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<i64>>();

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

fn get_next_dest_ranges(
    source_range: &SourceRange,
    maps: &HashMap<&str, Vec<MapRange>>,
) -> Vec<SourceRange> {
    let mut dest_ranges = Vec::new();
    let mut covered_source_ranges = HashSet::new();

    let map_name = &source_range.name;
    let map_key = maps.keys().find(|&name| name.starts_with(map_name));

    // Reached the end
    if map_key.is_none() {
        return Vec::new();
    }

    let map_key = map_key.unwrap();
    let new_source_name = map_key.split("-").last().unwrap().to_string();
    let map_ranges = maps.get(map_key).unwrap();
    assert!(!new_source_name.is_empty());

    println!("map_name {} map_key: {:?}", map_name, map_key);

    for map in map_ranges {
        if source_range.start > map.source_end || source_range.end < map.source_start {
            continue;
        }
        let mut dest_start = map.dest_start + (source_range.start - map.source_start);
        let mut dest_end = map.dest_start + (source_range.end - map.source_start);
        dest_start = max(dest_start, map.dest_start);
        dest_end = min(dest_end, map.dest_end);

        dest_ranges.push(SourceRange {
            start: dest_start,
            end: dest_end,
            name: new_source_name.clone(),
        });

        let covered_source_start = max(source_range.start, map.source_start);
        let covered_source_end = min(source_range.end, map.source_end);
        covered_source_ranges.insert(SourceRange {
            start: covered_source_start,
            end: covered_source_end,
            name: source_range.name.clone(),
        });
    }

    // Unchanged source range
    if dest_ranges.is_empty() {
        return vec![SourceRange {
            start: source_range.start,
            end: source_range.end,
            name: new_source_name,
        }];
    }

    let missing_dest_ranges =
        get_missing_dest_ranges(source_range, &covered_source_ranges, &new_source_name);

    println!("missing_dest_ranges: {:?}", missing_dest_ranges);
    dest_ranges.extend(missing_dest_ranges);

    dest_ranges
}

fn get_missing_dest_ranges(
    source_range: &SourceRange,
    covered_source_ranges: &HashSet<SourceRange>,
    new_source_name: &str,
) -> Vec<SourceRange> {
    let mut sorted_covered_source_ranges =
        covered_source_ranges.iter().collect::<Vec<&SourceRange>>();

    sorted_covered_source_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut missing_dest_ranges = Vec::new();

    let mut index = 0;
    while index < sorted_covered_source_ranges.len() {
        let current = sorted_covered_source_ranges[index];

        if current.start > source_range.start {
            missing_dest_ranges.push(SourceRange {
                start: source_range.start,
                end: current.start - 1,
                name: new_source_name.to_string(),
            });
        }

        let next = sorted_covered_source_ranges.get(index + 1);
        if next.is_none() {
            if current.end < source_range.end {
                missing_dest_ranges.push(SourceRange {
                    start: current.end + 1,
                    end: source_range.end,
                    name: new_source_name.to_string(),
                });
            }
            return missing_dest_ranges;
        }

        let next = next.unwrap();
        if current.end < next.start {
            missing_dest_ranges.push(SourceRange {
                start: current.end + 1,
                end: next.start - 1,
                name: new_source_name.to_string(),
            });
        }
        index += 1;
    }

    missing_dest_ranges
}

fn find_location_ranges_for_seed(
    seed: &SourceRange,
    maps: &HashMap<&str, Vec<MapRange>>,
) -> Vec<SourceRange> {
    let mut stack: Vec<SourceRange> = vec![seed.clone()];
    let mut locations = Vec::new();

    let mut counter = 0;
    while !stack.is_empty() {
        let current = stack.pop().unwrap();

        println!("====>> current: {:?}", current);
        let dest_ranges = get_next_dest_ranges(&current, maps);
        println!("~~~~~~~~~~~~~~~> dest_ranges: {:?}", dest_ranges);
        if dest_ranges.is_empty() {
            locations.push(current);
            continue;
        }
        stack.extend(dest_ranges);

        counter += 1;
        // if counter == 3 {
        //     panic!("counter: {}", counter);
        // }
    }

    locations
}

pub fn run() {
    let lines = read_file_lines("./src/day05/inputData.txt");
    let mut seeds = get_seeds(&lines[0]);
    println!("seeds: {:?}", seeds);

    let maps = read_maps(&lines[1..]);
    for (name, map) in maps.iter() {
        println!("------- {} ------", name);
        for range in map {
            println!("{:?}", range);
        }
    }

    let mut locations = Vec::new();

    for seed in seeds {
        // given some seed, find the final location ranges
        let locations_for_seed = find_location_ranges_for_seed(&seed, &maps);
        println!(
            "-------------- seed {:?} locations_for_seed: {:?} --------------",
            seed, locations_for_seed
        );
        locations.extend(locations_for_seed);
    }

    locations.sort_by(|a, b| a.start.cmp(&b.start));
    println!("locations: {:?}", locations);
    println!("min location: {:?}", locations.first().unwrap());
}

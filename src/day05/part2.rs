use crate::utils::read_file::read_file;
use std::{cmp::min, collections::HashSet, vec};

pub fn run() {
    let lines = read_file("./src/day05/testData.txt");

    let seeds = get_seeds_pairs(&lines[0]);

    let mut index = 1;
    let mut locations: Vec<(i64, i64)> = Vec::new();

    for (start, range) in seeds {
        println!("seed {} {}", start, range);
        let mut source = vec![(start, start + range - 1)];
        let mut dest = vec![(-1, -1)];

        while index < lines.len() {
            let line = &lines[index];
            if line.contains("map") {
                println!("----------- {} -----------", line);
                index += 1;
                dest = find_dest_nums(&mut index, &lines, &source);
                println!("source {:?} dest {:?}", source, dest);
                source = dest.clone();
            }

            index += 1;
        }

        // add all of the dests to locations
        locations.extend(dest);
        index = 1;
    }
    locations.sort_by(|a, b| a.0.cmp(&b.0));
    println!("locations {:?}", locations);
    // get min location number
    let min_location = locations.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();
    println!("min location number {}", min_location.0);
}

fn find_dest_nums(
    index: &mut usize,
    lines: &Vec<String>,
    source_nums: &Vec<(i64, i64)>,
) -> Vec<(i64, i64)> {
    let mut dest_nums: HashSet<(i64, i64)> = HashSet::new();
    for (source_start, source_end) in source_nums {
        println!(
            "FIND DEST NUM LOOP ---- source ({}, {})",
            source_start, source_end
        );
        let dest_nums_for_source = get_dest_nums_for_source(
            source_nums,
            source_start,
            source_end,
            &mut index.clone(),
            lines,
        );

        println!("dest_nums_for_source {:?}", dest_nums_for_source);
        dest_nums.extend(dest_nums_for_source);
    }

    if dest_nums.is_empty() {
        return source_nums.to_owned();
    }

    return dest_nums.into_iter().collect();
}

fn get_dest_nums_for_source(
    source_nums: &Vec<(i64, i64)>,
    source_start: &i64,
    source_end: &i64,
    index: &mut usize,
    lines: &Vec<String>,
) -> Vec<(i64, i64)> {
    let mut dest_nums: HashSet<(i64, i64)> = HashSet::new();
    let mut source_map_ranges: Vec<(i64, i64)> = Vec::new();

    while index < &mut lines.len() && !lines[*index].is_empty() {
        let line = &lines[*index];
        println!("line {}", line);
        *index += 1;

        let map_line = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i64>().ok())
            .collect::<Vec<i64>>();

        let map_start = map_line[1];
        let map_end = map_line[1] + map_line[2] - 1;
        let map_offset = map_line[0] - map_line[1];

        // map is completely outside of source
        if &map_end < source_start || &map_start > source_end {
            println!(
                "map ({}, {}) completely outside of source ({}, {})",
                map_start, map_end, source_start, source_end
            );
            continue;
        }

        if &map_start <= source_start && &map_end >= source_end {
            println!("map ({},{}) fully contains source", map_start, map_end);
            let overlap = (source_start.to_owned(), source_end.to_owned());
            let dest = (source_start + map_offset, source_end + map_offset);
            println!(
                "source_start {} source_end {} map_offset {}",
                source_start, source_end, map_offset
            );
            source_map_ranges.push(overlap);
            dest_nums.insert(dest);
            continue;
        }

        // map is fully inside source
        if &map_start >= source_start && &map_end <= source_end {
            println!("map fully inside source");
            let overlap = (map_start, map_end);
            let dest = (overlap.0 + map_offset, overlap.1 + map_offset);
            source_map_ranges.push(overlap);
            dest_nums.insert(dest);
            continue;
        }

        // map is partially inside source on the left
        if &map_start < source_start && &map_end >= source_start {
            println!("map partially inside source on the left");
            let overlap = (source_start.to_owned(), map_end.to_owned());
            let dest = (overlap.0 + map_offset, overlap.1 + map_offset);
            source_map_ranges.push(overlap);
            dest_nums.insert(dest);
            println!("dest {:?}", dest_nums);
            continue;
        }

        // map is partially inside source on the right
        if &map_start <= source_end && &map_end > source_end {
            println!("map partially inside source on the right");
            let overlap = (map_start.to_owned(), source_end.to_owned());
            let dest = (overlap.0 + map_offset, overlap.1 + map_offset);
            source_map_ranges.push(overlap);
            dest_nums.insert(dest);
            println!("dest {:?}", dest_nums);
            continue;
        }
    }
    if dest_nums.is_empty() {
        return vec![(source_start.to_owned(), source_end.to_owned())]
            .into_iter()
            .collect();
    }
    let missing_dest_nums: Vec<(i64, i64)> = get_missing_dest_nums(
        source_nums[0].0,
        source_nums[source_nums.len() - 1].1,
        source_map_ranges,
    );
    println!("missing_dest_nums {:?}", missing_dest_nums);

    dest_nums.extend(missing_dest_nums);

    return dest_nums.into_iter().collect();
}

fn get_missing_dest_nums(
    source_start: i64,
    source_end: i64,
    source_map_ranges: Vec<(i64, i64)>,
) -> Vec<(i64, i64)> {
    println!("source_map_ranges {:?}", source_map_ranges);
    let mut sorted_source_map_ranges = source_map_ranges.clone();
    sorted_source_map_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    println!("sorted {:?}", sorted_source_map_ranges);

    let mut missing_dest_nums: Vec<(i64, i64)> = Vec::new();

    if sorted_source_map_ranges[0].0 > source_start {
        missing_dest_nums.push((source_start, sorted_source_map_ranges[0].0 - 1));
    }

    for i in 0..sorted_source_map_ranges.len() - 1 {
        let current_end = sorted_source_map_ranges[i].1 + 1;
        let next_start = if i == sorted_source_map_ranges.len() - 1 {
            source_end
        } else {
            sorted_source_map_ranges[i + 1].0 - 1
        };

        if current_end < next_start {
            missing_dest_nums.push((current_end, next_start));
        }
    }

    missing_dest_nums
}

fn get_seeds_pairs(line: &str) -> Vec<(i64, i64)> {
    let numbers = line
        .split_whitespace()
        .skip(1)
        .filter_map(|num| num.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    numbers.chunks(2).map(|pair| (pair[0], pair[1])).collect()
}

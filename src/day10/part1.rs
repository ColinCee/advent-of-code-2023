use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::read_file_lines::read_file_lines;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

fn parse_line_to_graph(lines: &Vec<String>) -> (String, HashMap<String, HashSet<String>>) {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let mut start_id: String = "".to_string();
    let lines_len = lines.len();

    for (i, line) in lines.iter().enumerate() {
        let line_len = line.len();

        for j in 0..line_len {
            let c = line.chars().nth(j).unwrap();
            let current_id = format!("{}-{}", i, j);

            if c == 'S' {
                start_id = current_id.clone();
            }

            let mut children = vec![];

            match c {
                '|' => {
                    if i > 0 {
                        children.push(format!("{}-{}", i - 1, j));
                    }
                    if i + 1 < lines_len {
                        children.push(format!("{}-{}", i + 1, j));
                    }
                }
                '-' => {
                    if j + 1 < line_len {
                        children.push(format!("{}-{}", i, j + 1));
                    }
                    if j > 0 {
                        children.push(format!("{}-{}", i, j - 1));
                    }
                }
                'L' => {
                    if i > 0 {
                        children.push(format!("{}-{}", i - 1, j));
                    }
                    if j + 1 < line_len {
                        children.push(format!("{}-{}", i, j + 1));
                    }
                }
                'J' => {
                    if i > 0 {
                        children.push(format!("{}-{}", i - 1, j));
                    }
                    if j > 0 {
                        children.push(format!("{}-{}", i, j - 1));
                    }
                }
                '7' => {
                    if i + 1 < lines_len {
                        children.push(format!("{}-{}", i + 1, j));
                    }
                    if j > 0 {
                        children.push(format!("{}-{}", i, j - 1));
                    }
                }
                'F' => {
                    if i + 1 < lines_len {
                        children.push(format!("{}-{}", i + 1, j));
                    }
                    if j + 1 < line_len {
                        children.push(format!("{}-{}", i, j + 1));
                    }
                }
                _ => {}
            }

            // for each child of the node, add the current node as a child too
            for child_id in &children {
                graph
                    .entry(child_id.clone())
                    .or_insert_with(HashSet::new)
                    .insert(current_id.clone());
            }

            let child_hashset: HashSet<String> = children.into_iter().collect();
            println!(
                "i {} j {} inserted child_hashset: {:?}",
                i, j, child_hashset
            );

            graph
                .entry(current_id.clone())
                .or_insert_with(HashSet::new)
                .extend(child_hashset);
        }
    }

    println!("graph.len(): {}", graph.len());
    assert!(graph.len() == lines_len * lines_len);
    (start_id, graph)
}
fn debug_print_furthest_nodes(lines: &Vec<String>, counts: &HashMap<String, i64>) {
    println!("counts: {:?}", counts);
    for i in 0..lines.len() {
        let mut line = "".to_string();

        for j in 0..lines[i].len() {
            let current_id = format!("{}-{}", i, j);
            let current_count = counts.get(&current_id);
            let output = match current_count {
                Some(count) => format!("{}-", count.to_string()),
                None => ".-".to_string(),
            };
            line.push_str(&output);
        }
        println!("{}", line);
    }
}

fn count_steps_from_start(
    start_id: &String,
    graph: &HashMap<String, HashSet<String>>,
) -> HashMap<String, i64> {
    let mut current_counts: HashMap<String, i64> = HashMap::new();
    current_counts.insert(start_id.clone(), 0);

    let mut queue: VecDeque<String> = VecDeque::new();
    for child_id in graph.get(start_id).unwrap() {
        queue.push_back(child_id.clone());
    }

    while let Some(current_node_id) = queue.pop_front() {
        let current_node = graph.get(&current_node_id).unwrap();
        if current_counts.contains_key(&current_node_id) {
            continue;
        }

        for child_id in current_node {
            queue.push_back(child_id.clone());
        }

        let min_counts = current_node
            .iter()
            .map(|child_id| current_counts.get(child_id))
            .filter(|count| count.is_some())
            .map(|count| count.unwrap())
            .min();

        if min_counts.is_none() {
            println!("current_node {:?}", current_node);
            panic!("min_counts is none for node: {}", current_node_id);
        }

        let current_count = min_counts.unwrap() + 1;
        current_counts.insert(current_node_id.clone(), current_count);
    }

    // debug_print_furthest_nodes(&lines, &current_counts);
    current_counts
}

pub fn run() {
    let lines = read_file_lines("src/day10/inputData.txt");
    let (start_id, graph) = parse_line_to_graph(&lines);

    // println!("45-73 {:?}", graph.get("45-73"));
    // print all nodes with children
    // for (k, v) in &graph {
    //     if !v.is_empty() {
    //         println!("{}: {:?}", k, v);
    //     }
    // }

    let step_counts = count_steps_from_start(&start_id, &graph);
    debug_print_furthest_nodes(&lines, &step_counts);
    let max_steps = step_counts.values().max().unwrap();
    let max_node: String = step_counts
        .iter()
        .filter(|(_, v)| *v == max_steps)
        .map(|(k, _)| k.clone())
        .collect();

    println!("max_node: {:?}", max_node);
    println!("max_steps: {}", max_steps);
}

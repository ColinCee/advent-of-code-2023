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
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();

        for j in 0..line.len() {
            let c = line.chars().nth(j).unwrap();
            let current_id = format!("{}-{}", i, j);

            if c == 'S' {
                start_id = current_id.clone();
            }

            let children = match c {
                '|' => vec![format!("{}-{}", i - 1, j), format!("{}-{}", i + 1, j)],
                '-' => vec![format!("{}-{}", i, j + 1), format!("{}-{}", i, j - 1)],
                'L' => vec![format!("{}-{}", i - 1, j), format!("{}-{}", i, j + 1)],
                'J' => vec![format!("{}-{}", i - 1, j), format!("{}-{}", i, j - 1)],
                '7' => vec![format!("{}-{}", i + 1, j), format!("{}-{}", i, j - 1)],
                'F' => vec![format!("{}-{}", i + 1, j), format!("{}-{}", i, j + 1)],
                _ => vec![],
            };

            // for each child of the node, add the current node as a child too
            for child_id in &children {
                let child_node = graph.get_mut(child_id);
                match child_node {
                    Some(child_node) => {
                        child_node.insert(current_id.clone());
                    }
                    None => {}
                }
            }

            let child_hashset: HashSet<String> = children.iter().map(|s| s.clone()).collect();
            graph.insert(current_id.clone(), child_hashset);
        }
    }

    (start_id, graph)
}

fn debug_print_furthest_nodes(lines: &Vec<String>, counts: &HashMap<String, i32>) {
    println!("counts: {:?}", counts);
    for i in 0..lines.len() {
        let mut line = "".to_string();

        for j in 0..lines[i].len() {
            let current_id = format!("{}-{}", i, j);
            let current_count = counts.get(&current_id);
            let output = match current_count {
                Some(count) => format!("{}", count.to_string()),
                None => ".".to_string(),
            };
            line.push_str(&output);
        }
        println!("{}", line);
    }
}

fn count_steps_from_start(
    start_id: &String,
    graph: &HashMap<String, HashSet<String>>,
    lines: &Vec<String>,
) -> HashMap<String, i32> {
    let mut current_counts: HashMap<String, i32> = HashMap::new();
    current_counts.insert(start_id.clone(), 0);

    let mut queue: VecDeque<String> = VecDeque::new();
    for child_id in graph.get(start_id).unwrap() {
        queue.push_back(child_id.clone());
    }

    while let Some(current_node_id) = queue.pop_front() {
        let current_node = graph.get(&current_node_id).unwrap();

        // get the min from all the current nodes children
        let current_count = current_node
            .iter()
            .map(|child_id| current_counts.get(child_id).unwrap_or(&i32::MAX))
            .min()
            .unwrap()
            + 1;

        current_counts.insert(current_node_id.clone(), current_count);

        let children_to_visit: Vec<String> = current_node
            .iter()
            .filter(|child_id| !current_counts.contains_key(*child_id))
            .map(|child_id| child_id.clone())
            .collect();

        for child_id in children_to_visit {
            queue.push_back(child_id);
        }
    }

    debug_print_furthest_nodes(&lines, &current_counts);
    current_counts
}

pub fn run() {
    let lines = read_file_lines("src/day10/inputData.txt");

    println!("lines: {:?}", lines);

    let (start_id, graph) = parse_line_to_graph(&lines);
    // print all nodes with children
    for (k, v) in &graph {
        if !v.is_empty() {
            println!("{}: {:?}", k, v);
        }
    }

    let step_counts = count_steps_from_start(&start_id, &graph, &lines);
    let max_steps = step_counts.values().max().unwrap();
    let max_node: String = step_counts
        .iter()
        .filter(|(_, v)| *v == max_steps)
        .map(|(k, _)| k.clone())
        .collect();

    println!("max_node: {:?}", max_node);
    println!("max_steps: {}", max_steps);
}

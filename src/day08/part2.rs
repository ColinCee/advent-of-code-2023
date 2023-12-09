use core::num;
use std::collections::HashMap;

use regex::Regex;

use crate::utils::read_file_lines::read_file_lines;

#[derive(Debug, Clone)]
struct BinaryNode {
    name: String,
    left_child: Option<String>,
    right_child: Option<String>,
}

impl BinaryNode {
    fn into_boxed(self) -> Box<BinaryNode> {
        Box::new(self)
    }
}

fn build_graph(lines: &Vec<String>) -> HashMap<String, BinaryNode> {
    let re: Regex = Regex::new(r"\b\w+\b").unwrap();

    lines.iter().skip(2).fold(
        HashMap::new(),
        |mut map: HashMap<String, BinaryNode>, line| {
            let words: Vec<&str> = re.find_iter(line).map(|mat| mat.as_str()).collect();

            let current_node_name = words[0];
            let current_node = map.get(current_node_name);
            if (current_node).is_some() {
                panic!("Node already exists: {}", current_node_name);
            }
            map.insert(
                current_node_name.to_string(),
                BinaryNode {
                    name: current_node_name.to_string(),
                    left_child: Some(words[1].to_string()),
                    right_child: Some(words[2].to_string()),
                },
            );

            map
        },
    )
}

fn get_all_start_nodes<'a>(
    lines: &Vec<String>,
    graph: &'a HashMap<String, BinaryNode>,
) -> Vec<&'a BinaryNode> {
    lines
        .iter()
        .skip(2)
        .map(|line| line.split_whitespace().next().unwrap().to_string())
        .filter(|name| name.ends_with("A"))
        .map(|name| graph.get(&name).unwrap())
        .collect()
}

fn find_num_steps_to_z(
    instructions: &Vec<char>,
    start_node: &BinaryNode,
    graph: &HashMap<String, BinaryNode>,
) -> u64 {
    let mut steps: u64 = 0;

    let mut stack = Vec::new();
    stack.push(start_node);

    while !stack.is_empty() {
        let current_node = stack.pop().unwrap();
        if current_node.name.ends_with("Z") {
            return steps;
        }

        let current_instruction = instructions[(steps % instructions.len() as u64) as usize];

        match current_instruction {
            'L' => {
                let left_child_name = current_node.left_child.as_ref().unwrap();
                let node_name = graph.get(left_child_name).unwrap();
                stack.push(node_name)
            }
            'R' => {
                let right_child_name = current_node.right_child.as_ref().unwrap();
                let node_name = graph.get(right_child_name).unwrap();
                stack.push(node_name)
            }
            _ => panic!("Invalid instruction: {}", current_instruction),
        }

        steps += 1;
    }

    steps
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn run() {
    let lines = read_file_lines("src/day08/inputData.txt");

    let graph: HashMap<String, BinaryNode> = build_graph(&lines);

    let instruction_chars = &lines[0].chars().collect::<Vec<char>>();
    let current_nodes = get_all_start_nodes(&lines, &graph);

    let steps_required_for_nodes = current_nodes
        .iter()
        .map(|node| find_num_steps_to_z(&instruction_chars, node, &graph))
        .collect::<Vec<u64>>();

    // lowest common multiple of all steps required for each node
    let lcm = steps_required_for_nodes
        .iter()
        .fold(1, |acc, x| lcm(acc, *x));

    println!("LCM: {}", lcm);
}

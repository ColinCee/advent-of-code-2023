use std::collections::HashMap;

use regex::Regex;

use crate::utils::read_file_lines::read_file_lines;

#[derive(Debug)]
struct BinaryNode {
    name: String,
    left_child: Option<Box<BinaryNode>>,
    right_child: Option<Box<BinaryNode>>,
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

            if map.get(current_node_name).is_some() {
                // node already exists
                return map;
            }
            map.insert(
                current_node_name.to_string(),
                BinaryNode {
                    name: current_node_name.to_string(),
                    left_child: Some(
                        BinaryNode {
                            name: words[1].to_string(),
                            left_child: None,
                            right_child: None,
                        }
                        .into_boxed(),
                    ),
                    right_child: Some(
                        BinaryNode {
                            name: words[2].to_string(),
                            left_child: None,
                            right_child: None,
                        }
                        .into_boxed(),
                    ),
                },
            );

            map
        },
    )
}

pub fn run() {
    let lines = read_file_lines("src/day08/inputData.txt");

    let graph: HashMap<String, BinaryNode> = build_graph(&lines);

    let instruction_chars = &lines[0].chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut current = graph.get("AAA").unwrap();

    loop {
        if current.name == "ZZZ" {
            break;
        }
        let c = instruction_chars[i % instruction_chars.len()].to_string();
        let next_node: &Box<BinaryNode> = match c.as_str() {
            "L" => current.left_child.as_ref().unwrap(),
            "R" => current.right_child.as_ref().unwrap(),
            _ => panic!("Unknown instruction"),
        };
        current = graph.get(&next_node.name).unwrap();
        i += 1;
    }

    println!("{:?}", graph);
    println!("Count: {}", i);
}

use std::collections::HashMap;

pub fn run() {

    let input = std::fs::read_to_string("./src/day02/inputData.txt").unwrap();

    let lines = input.lines();

    let mut total = 0;
    for (i, line) in lines.enumerate() { 
        // get the max of each color
        let valid = is_game_valid(line);

        if valid {
            total += i + 1;
        }
    }
    println!("Total: {}", total);
}

fn get_max_color_counts(line: &str) -> HashMap<String, i32> {

    let mut color_counts: HashMap<String, i32> = HashMap::new();
    color_counts.insert("red".to_string(), 0);
    color_counts.insert("green".to_string(), 0);
    color_counts.insert("blue".to_string(), 0);

    let color_counts_str = line.split(":").nth(1).unwrap();
    // delete the semicolon
    let colors = color_counts_str.replace(";", ",");

    for color in colors.split(",") {
        let mut parts = color.split_whitespace();
        let color_count = parts.next().unwrap().parse::<i32>().unwrap();
        let color_name = parts.next().unwrap().to_string();

        if color_counts.get(&color_name).unwrap() < &color_count {
            color_counts.insert(color_name, color_count);
        }
    }

    return color_counts;
}

fn is_game_valid(line: &str) -> bool {
    let color_counts = get_max_color_counts(line);
    let num_red = color_counts.get("red").unwrap();
    let num_green = color_counts.get("green").unwrap();
    let num_blue = color_counts.get("blue").unwrap();

    // println!("{} {} {}", num_red, num_green, num_blue);
    return num_red <= &12 && num_green <= &13 && num_blue <= &14;
}

use crate::utils::read_file_lines::read_file_lines;

pub fn run() {
    let lines = read_file_lines("src/day08/testData.txt");
    println!("lines: {:?}", lines);
}

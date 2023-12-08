use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_file_lines(pah: &str) -> Vec<String> {
    let path = Path::new(pah);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}

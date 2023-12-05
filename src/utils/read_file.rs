use std::{
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_file(pah: &str) -> Vec<String> {
    let path = Path::new(pah);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}
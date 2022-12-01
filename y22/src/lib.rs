use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file!");
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Unable to read line"))
        .into_iter()
        .collect()
}

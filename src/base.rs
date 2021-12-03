use std::fs::File;
use std::io::{Result, BufRead, BufReader};

pub trait DayExecutable {
    fn execute(&self);
}

pub fn read_lines() -> Result<Vec<String>> {
    File::open("./data/day1.txt").map(|f| BufReader::new(f).lines().into_iter().map(|l| l.unwrap()).collect())
}
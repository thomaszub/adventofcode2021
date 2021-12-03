use std::fs::File;
use std::io::{Result, BufRead, BufReader};
use std::path::Path;

pub trait DayExecutable {
    fn execute(&self);
}

pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    File::open(path).map(|f| BufReader::new(f).lines().into_iter().map(|l| l.unwrap()).collect())
}
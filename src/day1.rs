use itertools::Itertools;
use crate::base::{DayExecutable, read_lines};

pub struct Day1Executable {}

impl DayExecutable for Day1Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day1.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines)
        }
    }
}

impl Day1Executable {
    fn process(&self, lines: Vec<String>) {
        let result = lines.iter().map(|l| l.parse::<u32>().unwrap()).into_iter().tuple_windows::<(_,_)>().map(|(i1, i2)| if i1 < i2 { 1 } else { 0 } ).fold(0, |sum, v| sum + v);
        println!("Result: {}", result);
    }
}

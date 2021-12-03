use crate::base::read_lines;
use crate::day2::Direction::{Down, Forward, Up};
use crate::DayExecutable;

#[derive(Debug)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub struct Day2Executable {}

impl DayExecutable for Day2Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day2.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

impl Day2Executable {
    fn process(&self, lines: Vec<String>) {
        let (mut pos, mut depth, mut aim) = (0, 0, 0);
        lines
            .iter()
            .map(|l| create_direction(l).unwrap())
            .for_each(|d| match d {
                Forward(v) => {
                    pos += v;
                    depth += aim * v;
                }
                Down(v) => aim += v,
                Up(v) => aim -= v,
            });
        println!("Result: {}", pos * depth);
    }
}

fn create_direction(value: &String) -> Result<Direction, String> {
    let splitted: Vec<&str> = value.split(' ').collect();
    match (splitted.get(0), splitted.get(1)) {
        (Some(&dir), Some(&val)) => build_direction(dir, val),
        _ => Err(format!("Value not properly formatted: {}", value)),
    }
}

fn build_direction(dir: &str, val: &str) -> Result<Direction, String> {
    match (dir, val.parse::<u32>()) {
        ("forward", Ok(v) ) => Ok(Forward(v)),
        ("down", Ok(v)) => Ok(Down(v)),
        ("up", Ok(v)) => Ok(Up(v)),
        _ => Err(format!("Illegal direction and/or value: {}, {}", dir, val)),
    }
}

use crate::base::read_lines;
use crate::DayExecutable;

pub struct Day3Executable {}

impl DayExecutable for Day3Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day3.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

impl Day3Executable {
    fn process(&self, lines: Vec<String>) {}
}

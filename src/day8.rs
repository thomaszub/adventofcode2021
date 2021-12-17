use crate::base::read_lines;
use crate::DayExecutable;

pub struct Day8Executable {}

impl DayExecutable for Day8Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day8.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

impl Day8Executable {
    fn process(&self, lines: Vec<String>) {}
}

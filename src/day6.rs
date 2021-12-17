use crate::base::read_lines;
use crate::DayExecutable;

pub struct Day6Executable {}

impl DayExecutable for Day6Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day6.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

impl Day6Executable {
    fn process(&self, lines: Vec<String>) {

    }
}

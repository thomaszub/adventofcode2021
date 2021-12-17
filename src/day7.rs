use crate::base::read_lines;
use crate::DayExecutable;

pub struct Day7Executable {}

impl DayExecutable for Day7Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day7.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

impl Day7Executable {
    fn process(&self, lines: Vec<String>) {

    }
}

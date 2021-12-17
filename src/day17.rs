use crate::base::read_lines;
use crate::DayExecutable;

pub struct Day17Executable {}

impl DayExecutable for Day17Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day17.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

impl Day17Executable {
    fn process(&self, lines: Vec<String>) {

    }
}

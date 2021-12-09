use crate::base::read_lines;
use crate::DayExecutable;

pub struct Day4Executable {}

impl DayExecutable for Day4Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day4.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

impl Day4Executable {
    fn process(&self, lines: Vec<String>) {
        let numbers: Vec<u32> = lines.get(0).unwrap().split(',').map(|s| s.parse::<u32>().unwrap()).collect();
        println!("{:?}", numbers);
        let boardLines = &lines[2..];
        println!("{:?}", boardLines);
    }
}

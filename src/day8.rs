use std::collections::HashMap;
use crate::base::read_lines;
use crate::DayExecutable;

pub struct Day8Executable {

}

impl DayExecutable for Day8Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day8.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}



#[derive(Debug)]
struct InputOutput(Vec<String>, Vec<String>);

impl Day8Executable {
    fn process(&self, lines: Vec<String>) {
        let ios = get_input_output(lines);
        let count = ios.iter()
            .map(|io| decode(io))
            .fold(0, |agg,v| agg +v)
            ;
        println!("Count: {}", count);
    }
}

fn get_input_output(lines: Vec<String>) -> Vec<InputOutput> {
    lines.iter()
        .map(|l| split_input_output(l))
        .collect()
}

fn split_input_output(line: &String) -> InputOutput {
    let splitted: Vec<Vec<String>> = line
        .split("|")
        .map(|s| s.trim())
        .map(|s| s.split(" ").map(|s| s.to_string()).collect())
        .take(2)
        .collect();
    InputOutput(
        splitted.get(0).unwrap().clone(),
        splitted.get(1).unwrap().clone(),
    )
}

fn decode(io: &InputOutput) -> usize {
    let len_to_digit: HashMap<usize, i32> = HashMap::from([
        (2, 1),
        (4, 4),
        (3, 7),
        (7, 8),
    ]);
    io.1.iter().map(|v| len_to_digit.get(&v.len())).filter(|p| p.is_some()).count()
}
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
        let mut state = get_initial_state(lines.get(0).unwrap());
        for _ in 0..80 {
            let mut new_fish = 0;
            for s in state.iter_mut() {
                match s {
                    0 => {
                        *s = 6;
                        new_fish += 1;
                    },
                    _ => *s -= 1
                }
            }
            for _ in 0..new_fish {
                state.push(8);
            }
        }
        println!("Count: {}", state.len());
    }
}

fn get_initial_state(line: &String) -> Vec<u32> {
    line.split(",").map(|s| s.parse().unwrap()).collect()
}

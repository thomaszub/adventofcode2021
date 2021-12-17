use crate::base::read_lines;
use crate::DayExecutable;
use itertools::Itertools;
use std::collections::HashMap;

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
        state.sort();
        let mut timer_map: HashMap<u64, u64> = state
            .iter()
            .group_by(|s| s.clone())
            .into_iter()
            .map(|(k, v)| (*k, v.count() as u64))
            .collect();
        for _ in 0..256 {
            let mut new_timer_map: HashMap<u64, u64> = HashMap::new();
            for entry in timer_map {
                let key = entry.0;
                let value = entry.1;
                if key == 0 {
                    *new_timer_map.entry(6).or_insert_with(|| 0) += value;
                    *new_timer_map.entry(8).or_insert_with(|| 0) += value;
                } else {
                    *new_timer_map.entry(key - 1).or_insert_with(|| 0) += value;
                }
            }
            timer_map = new_timer_map;
        }
        let count = timer_map.iter().map(|(_, v)| v).fold(0, |agg, v| agg + v);
        println!("Count: {}", count);
    }
}

fn get_initial_state(line: &String) -> Vec<u64> {
    line.split(",").map(|s| s.parse().unwrap()).collect()
}

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
        let positions = get_positions(lines.get(0).unwrap());
        let pos_min = positions.iter().min().unwrap().clone();
        let pos_max = positions.iter().max().unwrap().clone();
        let mut opt_fuel = calc_fuel(&positions, pos_max);
        for pos in pos_min..pos_max {
            let fuel = calc_fuel(&positions, pos);
            if fuel < opt_fuel {
                opt_fuel = fuel
            }
        }
        println!("Optimal fuel: {}", opt_fuel);
    }
}

fn calc_fuel(positions: &Vec<i32>, ref_pos: i32) -> i32 {
    positions
        .iter()
        .fold(0, |agg, v| agg + fuel_cost(v, ref_pos))
}

fn fuel_cost(pos: &i32, ref_pos: i32) -> i32 {
    let n = (pos - ref_pos).abs();
    n * (n + 1) / 2
}

fn get_positions(line: &String) -> Vec<i32> {
    line.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}

use crate::base::read_lines;
use crate::DayExecutable;
use regex::{Captures, Regex};
use std::cmp::{max, min};

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

#[derive(Debug)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl TargetArea {
    fn in_area(&self, x: i32, y: i32) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

impl Day17Executable {
    fn process(&self, lines: Vec<String>) {
        let target_area = get_target_area(lines.get(0).unwrap());
        println!("Target area: {:?}", target_area);
        let mut count = 0;
        for v_y_0 in -1000..1000 {
            for abs_v_x_0 in 1..1000 {
                let v_x_0 = target_area.x_min.signum() * abs_v_x_0;
                let mut step = 1;
                loop {
                    let x = calc_x(step, v_x_0);
                    let y = calc_y(step, v_y_0);
                    if target_area.in_area(x, y) {
                        count += 1;
                        break;
                    }
                    if x > target_area.x_max || y < target_area.y_min {
                        break;
                    }
                    step += 1;
                }
            }
        }
        println!("Count: {}", count);
    }
}

fn get_target_area(line: &String) -> TargetArea {
    let reg_x = Regex::new(r"x=(-?[0-9]+)\.\.(-?[0-9]+)").unwrap();
    let caps_x = reg_x.captures(line.as_str()).unwrap();
    let x1 = get_i32_from_capture(&caps_x, 1);
    let x2 = get_i32_from_capture(&caps_x, 2);
    let reg_y = Regex::new(r"y=(-?[0-9]+)\.\.(-?[0-9]+)").unwrap();
    let caps_y = reg_y.captures(line.as_str()).unwrap();
    let y1 = get_i32_from_capture(&caps_y, 1);
    let y2 = get_i32_from_capture(&caps_y, 2);

    TargetArea {
        x_min: min(x1, x2),
        x_max: max(x1, x2),
        y_min: min(y1, y2),
        y_max: max(y1, y2),
    }
}

fn get_i32_from_capture(cap: &Captures, group: usize) -> i32 {
    cap.get(group)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .unwrap()
}

fn calc_y(step: i32, v_y_0: i32) -> i32 {
    return step * (2 * v_y_0 - step + 1) / 2;
}

fn calc_x(step: i32, v_x_0: i32) -> i32 {
    let mins = min(step, v_x_0.abs());
    return mins * (2 * v_x_0.abs() - mins + 1) / 2;
}

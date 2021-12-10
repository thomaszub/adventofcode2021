use std::cmp::{max, min};
use std::collections::HashMap;
use crate::base::read_lines;
use crate::DayExecutable;

pub struct Day5Executable {}

impl DayExecutable for Day5Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day5.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point{
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PointPair{
    p1: Point,
    p2: Point,
}

impl Day5Executable {
    fn process(&self, lines: Vec<String>) {
        let point_pairs: Vec<PointPair> = lines
            .iter()
            .map(|s| s.split(" -> ").collect())
            .map(|p: Vec<&str>| PointPair{p1: create_point(p[0]), p2: create_point(p[1])})
            .collect();
        let mut map: HashMap<Point, u32> = HashMap::new();
        for point_pair in point_pairs {
            let p1 = point_pair.p1;
            let p2 = point_pair.p2;
            let min_x = min(p1.x, p2.x);
            let max_x = max(p1.x, p2.x);
            let min_y = min(p1.y, p2.y);
            let max_y = max(p1.y, p2.y);

            if p1.x != p2.x && p1.y == p2.y {
                for x in min_x..(max_x + 1) {
                    *map.entry(Point{x, y: p1.y}).or_insert_with(||0) += 1;
                }
                continue;
            }
            if p1.x == p2.x && p1.y != p2.y {
                for y in min_y..(max_y + 1) {
                    *map.entry(Point{x: p1.x, y}).or_insert_with(||0) += 1;
                }
                continue;
            }
            let diff_x = max_x - min_x;
            let diff_y = max_y - min_y;
            if diff_x == diff_y {
                let step_x = (p2.x  - p1.x) / diff_x;
                let step_y = (p2.y  - p1.y) / diff_x;
                for id in 0..(diff_x + 1) {
                    let point = Point{x: p1.x + id*step_x, y: p1.y + id*step_y};
                    *map.entry(point).or_insert_with(||0) += 1;
                }
            }
        }
        let at_least_two_lines = map.iter().map(|e| e.1).filter(|v| **v >= 2).count();
        println!("At least two lines: {:?}", at_least_two_lines);
    }
}

fn create_point(str: &str) -> Point {
    let splitted: Vec<&str> = str.split(",").collect();
    Point{x: splitted[0].parse().unwrap(), y: splitted[1].parse().unwrap()}
}

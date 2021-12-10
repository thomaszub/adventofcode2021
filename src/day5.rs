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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(u32, u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct PointPair(Point, Point);

impl Day5Executable {
    fn process(&self, lines: Vec<String>) {
        let point_pairs: Vec<PointPair> = lines.iter().map(|s| s.split(" -> ").collect()).map(|p: Vec<&str>| PointPair(create_point(p[0]), create_point(p[1]))).collect();
        println!("{:?}", point_pairs);
    }
}

fn create_point(str: &str) -> Point {
    let splitted: Vec<&str> = str.split(",").collect();
    Point(splitted[0].parse().unwrap(), splitted[1].parse().unwrap())
}
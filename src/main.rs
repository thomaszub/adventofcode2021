mod base;
mod day1;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use crate::base::DayExecutable;
use crate::day1::Day1Executable;
use crate::day17::Day17Executable;
use crate::day2::Day2Executable;
use crate::day3::Day3Executable;
use crate::day4::Day4Executable;
use crate::day5::Day5Executable;
use crate::day6::Day6Executable;
use crate::day7::Day7Executable;
use crate::day8::Day8Executable;
use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let executor: Option<Box<dyn DayExecutable>> = match args.get(1) {
        None => None,
        Some(name) => match name.as_str() {
            "day1" => Some(Box::new(Day1Executable {})),
            "day2" => Some(Box::new(Day2Executable {})),
            "day3" => Some(Box::new(Day3Executable {})),
            "day4" => Some(Box::new(Day4Executable {})),
            "day5" => Some(Box::new(Day5Executable {})),
            "day6" => Some(Box::new(Day6Executable {})),
            "day7" => Some(Box::new(Day7Executable {})),
            "day8" => Some(Box::new(Day8Executable {})),
            "day17" => Some(Box::new(Day17Executable {})),
            _ => None,
        },
    };
    match executor {
        None => println!("No known day name provided"),
        Some(exec) => exec.execute(),
    }
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

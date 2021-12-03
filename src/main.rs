mod base;
mod day1;

use std::env;
use crate::base::DayExecutable;
use crate::day1::Day1Executable;

fn main() {
    let args: Vec<String> = env::args().collect();
    let executor: Option<Box<dyn DayExecutable>> = match args.get(1) {
        None => None,
        Some(name) => {
            match name.as_str() {
                "day1" => Some(Box::new(Day1Executable{})),
                _ => None
            }
        }
    };
    match executor {
        None => {println!("No known day name provided")}
        Some(exec) => {exec.execute()}
    }
}

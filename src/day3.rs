use crate::base::read_lines;
use crate::DayExecutable;
use std::str::Chars;

pub struct Day3Executable {}

impl DayExecutable for Day3Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day3.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

impl Day3Executable {
    fn process(&self, lines: Vec<String>) {
        let size = lines.len();
        let bits: Vec<Vec<u32>> = lines.iter().map(|l| l.chars()).map(chars_to_ints).collect();
        let most_common_bits: Vec<u8> = bits.iter()
            .fold(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], sum_vec)
            .iter()
            .map(|v| if 2 * v > size as u32 { 1 } else { 0 })
            .collect();
        let least_common_bits: Vec<u8> = most_common_bits.iter().map(|b| 1 - b).collect();
        let gamma_rate = bits_to_int(most_common_bits);
        let epsilon_rate = bits_to_int(least_common_bits);
        println!("Gamma rate: {}", gamma_rate);
        println!("Epsilon rate: {}", epsilon_rate);
        println!("Power consumption: {}", gamma_rate * epsilon_rate);
    }
}

fn chars_to_ints(chars: Chars) -> Vec<u32> {
    chars.map(|v| v.to_digit(f32::RADIX).unwrap()).collect()
}

fn sum_vec(mut agg: Vec<u32>, val: &Vec<u32>) -> Vec<u32> {
    for i in 0..12 {
        agg[i] += val[i];
    }
    agg
}

fn bits_to_int(bits: Vec<u8>) -> u32 {
    bits.iter().fold(0, |agg, bit| agg * 2 + *bit as u32)
}

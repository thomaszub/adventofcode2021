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
        let bits: Vec<Vec<u8>> = lines.iter().map(|l| l.chars()).map(chars_to_ints).collect();
        let most_common_bits: Vec<u8> = calc_most_common_bits(&bits);
        let least_common_bits: Vec<u8> = invert_bits(&most_common_bits);
        let gamma_rate = bits_to_int(&most_common_bits);
        let epsilon_rate = bits_to_int(&least_common_bits);
        let oxygen_rating = calc_bit_criteria(&bits, calc_most_common_bits);
        let co2_rating = calc_bit_criteria(&bits, |bits| {
            invert_bits(&calc_most_common_bits(&bits))
        });

        println!("Gamma rate: {}", gamma_rate);
        println!("Epsilon rate: {}", epsilon_rate);
        println!("Power consumption: {}", gamma_rate * epsilon_rate);
        println!("Oxygen rating: {}", oxygen_rating);
        println!("CO2 rating: {}", co2_rating);
        println!("Life support rating: {}", oxygen_rating * co2_rating);
    }
}

fn calc_most_common_bits(bits: &Vec<Vec<u8>>) -> Vec<u8> {
    let size = bits.len();
    bits.iter()
        .fold(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], sum_vec)
        .iter()
        .map(|v| if 2 * v >= size as u32 { 1 } else { 0 })
        .collect()
}

fn invert_bits(bits: &Vec<u8>) -> Vec<u8> {
    bits.iter().map(|b| 1 - b).collect()
}

fn calc_bit_criteria<F>(bits: &Vec<Vec<u8>>, remaining_comp_bits_fn: F) -> u32
where
    F: Fn(&Vec<Vec<u8>>) -> Vec<u8>,
{
    let mut rem_bits = bits.clone();
    let mut rem_comp_bits = remaining_comp_bits_fn(&bits);
    for id in 0..12 {
        rem_bits.retain(|v| v[id] == rem_comp_bits[id]);
        if rem_bits.len() == 1 {
            break;
        }
        rem_comp_bits = remaining_comp_bits_fn(&rem_bits);
    }
    bits_to_int(rem_bits.first().unwrap())
}

fn chars_to_ints(chars: Chars) -> Vec<u8> {
    chars
        .map(|v| v.to_digit(f32::RADIX).unwrap() as u8)
        .collect()
}

fn sum_vec(mut agg: Vec<u32>, val: &Vec<u8>) -> Vec<u32> {
    for i in 0..12 {
        agg[i] += val[i] as u32;
    }
    agg
}

fn bits_to_int(bits: &Vec<u8>) -> u32 {
    bits.iter().fold(0, |agg, bit| agg * 2 + *bit as u32)
}

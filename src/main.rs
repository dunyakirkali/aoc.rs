#![feature(array_windows)]
use std::fs;

mod day1;

pub use crate::day1::day1::part1;
pub use crate::day1::day1::part2;

fn day1() {
    let file_path = String::from("priv/day1/input.txt");
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    part1(&input);
    part2(&input);
}

fn main() {
    day1();
}

#![feature(array_windows)]
use std::fs;

mod day1;
mod day2;

pub use crate::day1::day1::part1 as day1_part1;
pub use crate::day1::day1::part2 as day1_part2;

pub use crate::day2::day2::part1 as day2_part1;
pub use crate::day2::day2::part2 as day2_part2;

fn day1() {
    let file_path = String::from("priv/day1/input.txt");
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    day1_part1(&input);
    day1_part2(&input);
}

fn day2() {
    let file_path = String::from("priv/day2/input.txt");
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    day2_part1(&input);
    day2_part2(&input);
}

fn main() {
    day1();
    day2();
}

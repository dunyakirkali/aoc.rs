#![feature(array_windows)]
use std::fs;

fn day1a() {
    let file_path = String::from("priv/day1.txt");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
     println!(
        "{:?}",
        contents
            .lines()
            .map(|n| n.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
            .array_windows::<2>()
            .filter(|[a, b]| a < b)
            .count()
     );
}

fn day1b() {
    let file_path = String::from("priv/day1.txt");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
     println!(
        "{:?}",
        contents
            .lines()
            .map(|n| n.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
            .array_windows::<3>()
            .map(|list| list.iter().sum::<u16>())
            .collect::<Vec<u16>>()
            .array_windows::<2>()
            .filter(|[a, b]| a < b)
            .count()
     );
}

fn main() {
    day1a();
    day1b();
}

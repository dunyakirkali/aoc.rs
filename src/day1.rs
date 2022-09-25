pub mod day1 {
    pub fn part1(input: &String) {
        println!(
            "{:?}",
            input
                .lines()
                .map(|n| n.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
                .array_windows::<2>()
                .filter(|[a, b]| a < b)
                .count()
        );
    }

    pub fn part2(input: &String) {
        println!(
            "{:?}",
            input
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
}

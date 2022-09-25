pub mod day2 {
    use std::str::FromStr;

    enum Action {
        Forward(u32),
        Up(u32),
        Down(u32),
    }

    impl FromStr for Action {
        type Err = ();

        fn from_str(input: &str) -> Result<Action, Self::Err> {
            let mut action_amount = input.split_whitespace();

            match action_amount.next().unwrap() {
                "forward" => Ok(Action::Forward(
                    action_amount.next().unwrap().parse::<u32>().unwrap(),
                )),
                "up" => Ok(Action::Up(
                    action_amount.next().unwrap().parse::<u32>().unwrap(),
                )),
                "down" => Ok(Action::Down(
                    action_amount.next().unwrap().parse::<u32>().unwrap(),
                )),
                _ => Err(()),
            }
        }
    }

    pub fn part1(input: &String) {
        let (x, y) =
            input
                .lines()
                .map(|n| Action::from_str(n).unwrap())
                .fold((0, 0), |(x, y), action| match action {
                    Action::Forward(amount) => (x + amount, y),
                    Action::Up(amount) => (x, y - amount),
                    Action::Down(amount) => (x, y + amount),
                });
        println!("{:?}", x * y);
    }

    pub fn part2(input: &String) {
        let (x, y, _aim) = input.lines().map(|n| Action::from_str(n).unwrap()).fold(
            (0, 0, 0),
            |(x, y, aim), action| match action {
                Action::Forward(amount) => (x + amount, y + aim * amount, aim),
                Action::Up(amount) => (x, y, aim - amount),
                Action::Down(amount) => (x, y, aim + amount),
            },
        );
        println!("{:?}", x * y);
    }
}

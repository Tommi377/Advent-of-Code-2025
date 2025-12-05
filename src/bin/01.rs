use std::fs;

fn main() {
    let input = fs::read_to_string("data/input/01").expect("File not found");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut res = 0;
    let mut current = 50;
    for line in input.lines() {
        let diff = match line.split_at_checked(1) {
            Some(("L", amount)) => -amount.parse::<i32>().unwrap(),
            Some(("R", amount)) => amount.parse::<i32>().unwrap(),
            _ => continue,
        };
        current = (current + diff).rem_euclid(100);
        if current == 0 {
            res += 1;
        }
    }
    res
}

fn part2(input: &str) -> i32 {
    let mut res: i32 = 0;
    let mut current: i32 = 50;
    for line in input.lines() {
        let diff: i32 = match line.split_at_checked(1) {
            Some(("L", amount)) => -amount.parse::<i32>().unwrap(),
            Some(("R", amount)) => amount.parse::<i32>().unwrap(),
            _ => continue,
        };
        res += (diff / 100).abs();
        let diff = diff % 100;

        if current != 0 && (current + diff <= 0 || current + diff >= 100) {
            res += 1;
        }

        current = (current + diff).rem_euclid(100);
    }
    res
}

#[cfg(test)]
mod test_day1 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/01");
    const INPUT: &str = include_str!("../../data/input/01");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 1182);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 6907);
    }
}

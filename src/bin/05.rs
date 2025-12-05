use std::fs;

fn main() {
    let input = fs::read_to_string("data/input/05").expect("File not found");
    println!("{}", part1(&input));
    // println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut lines_iter = input.lines();

    let ranges = lines_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|str| match str.split_once('-') {
            Some((left, right)) => (
                left.parse::<u64>().expect("Invalid number"),
                right.parse::<u64>().expect("Invalid number"),
            ),
            _ => panic!("Invalid format: {}", str),
        })
        .collect::<Vec<_>>();

    lines_iter
        .map(|id| id.parse::<u64>().expect("Invalid number"))
        .filter(|id| {
            for (left, right) in &ranges {
                if id >= left && id <= right {
                    return true;
                }
            }
            false
        })
        .count() as u64
}

fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod test_day5 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/05");
    const INPUT: &str = include_str!("../../data/input/05");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 868);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 14);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 8713);
    }
}

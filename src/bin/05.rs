use std::fs;

fn main() {
    let input = fs::read_to_string("data/input/05").expect("File not found");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
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
    let mut starts: Vec<u64> = Vec::new();
    let mut ends: Vec<u64> = Vec::new();

    input
        .lines()
        .take_while(|line| !line.is_empty())
        .for_each(|str| {
            match str.split_once('-') {
                Some((left, right)) => (
                    starts.push(left.parse::<u64>().expect("Invalid number")),
                    ends.push(right.parse::<u64>().expect("Invalid number")),
                ),
                _ => panic!("Invalid format: {}", str),
            };
        });

    let mut result: u64 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    starts.sort_unstable();
    ends.sort_unstable();

    while i < starts.len() && j < ends.len() {
        let mut next_i = i + 1;
        while next_i < starts.len() && j + 1 < ends.len() && starts[next_i] <= ends[j] {
            next_i += 1;
            j += 1;
        }

        result += ends[j] - starts[i] + 1;
        i = next_i;
        j += 1;
    }
    result
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
        assert_eq!(part2(INPUT), 354143734113772);
    }
}

use std::fs;

fn main() {
    let input = fs::read_to_string("data/input/02").expect("File not found");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut res = 0;
    for line in input.split(',') {
        match line.trim().split_once('-') {
            Some((first, last)) => {
                let first: u64 = first
                    .parse()
                    .unwrap_or_else(|_| panic!("{} should be a digit", first));
                let last: u64 = last
                    .parse()
                    .unwrap_or_else(|_| panic!("{} should be a digit", last));

                for num in first..=last {
                    let digits: Vec<char> = num.to_string().chars().collect();
                    if !digits.len().is_multiple_of(2) {
                        continue;
                    }

                    let mid = digits.len() / 2;
                    let mut success = true;
                    for i in 0..mid {
                        if digits[i] != digits[mid + i] {
                            success = false;
                            break;
                        }
                    }

                    res += if success { num } else { 0 };
                }
            }
            None => continue,
        };
    }
    res
}

fn part2(input: &str) -> u64 {
    let mut res = 0;
    for line in input.split(',') {
        match line.trim().split_once('-') {
            Some((first, last)) => {
                let first: u64 = first
                    .parse()
                    .unwrap_or_else(|_| panic!("{} should be a digit", first));
                let last: u64 = last
                    .parse()
                    .unwrap_or_else(|_| panic!("{} should be a digit", last));

                for num in first..=last {
                    let digits: Vec<char> = num.to_string().chars().collect();
                    for count in 2..=digits.len() {
                        if !digits.len().is_multiple_of(count) {
                            continue;
                        }

                        let mid = digits.len() / count;
                        let mut success = true;
                        for i in 0..mid {
                            for j in 1..count {
                                if digits[i] != digits[j * mid + i] {
                                    success = false;
                                    break;
                                }
                            }
                            if !success {
                                break;
                            }
                        }

                        if success {
                            res += num;
                            break;
                        }
                    }
                }
            }
            None => continue,
        };
    }
    res
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/02");
    const INPUT: &str = include_str!("../../data/input/02");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 24043483400);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 4174379265);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 38262920235);
    }
}

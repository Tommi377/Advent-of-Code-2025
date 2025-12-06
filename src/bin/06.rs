use std::fs;

fn main() {
    let input = fs::read_to_string("data/input/06").expect("File not found");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut accumulated: Vec<u64> = Vec::new();

    let ops = input
        .lines()
        .find(|line| line.starts_with(['+', '*']))
        .unwrap()
        .split_whitespace()
        .map(|token| token.chars().next().unwrap())
        .inspect(|op| {
            if *op == '+' {
                accumulated.push(0);
            } else {
                accumulated.push(1);
            }
        })
        .collect::<Vec<char>>();

    input
        .lines()
        .take_while(|line| !line.starts_with(['+', '*']))
        .for_each(|line| {
            line.split_whitespace().enumerate().for_each(|(i, token)| {
                if let Ok(num) = token.parse::<u64>() {
                    if ops[i] == '+' {
                        accumulated[i] += num;
                    } else {
                        accumulated[i] *= num;
                    }
                }
            });
        });

    accumulated.iter().sum()
}

fn part2(input: &str) -> u64 {
    let mut accumulated: Vec<u64> = Vec::new();
    let mut lengths: Vec<usize> = Vec::new();

    let number_lines = input
        .lines()
        .take_while(|line| !line.starts_with(['+', '*']))
        .collect::<Vec<_>>();

    let ops = input
        .lines()
        .find(|line| line.starts_with(['+', '*']))
        .unwrap()
        .split_whitespace()
        .map(|token| token.chars().next().unwrap())
        .inspect(|op| {
            if *op == '+' {
                accumulated.push(0);
            } else {
                accumulated.push(1);
            }
        })
        .collect::<Vec<char>>();

    let mut prev = 0;
    input
        .lines()
        .find(|line| line.starts_with(['+', '*']))
        .unwrap()
        .chars()
        .enumerate()
        .skip(1)
        .for_each(|(i, c)| {
            if c == '+' || c == '*' {
                lengths.push(i - prev - 1);
                prev = i
            };
        });
    lengths.push(number_lines[0].len() - prev);

    let mut current_index: usize = 0;
    for (op_i, length) in lengths.iter().enumerate() {
        for i in 0..*length {
            let mut num: u64 = 0;
            for line in number_lines.iter() {
                let char_byte: u8 = line.as_bytes()[current_index + i];
                if char_byte.is_ascii_digit() {
                    num *= 10;
                    num += (char_byte - b'0') as u64;
                }
            }
            let i = lengths.iter().position(|&l| l == *length).unwrap();
            if ops[op_i] == '+' {
                accumulated[op_i] += num;
            } else {
                accumulated[op_i] *= num;
            }
        }
        current_index += length + 1;
    }

    accumulated.iter().sum()
}

#[cfg(test)]
mod test_day5 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/06");
    const INPUT: &str = include_str!("../../data/input/06");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 4277556);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 5784380717354);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 3263827);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 7996218225744);
    }
}

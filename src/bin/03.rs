use std::fs;

fn main() {
    let input = fs::read_to_string("data/input/03").expect("File not found");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(parse_digits)
        .map(|digits| calculate_battery_value(&digits, 2))
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(parse_digits)
        .map(|digits| calculate_battery_value(&digits, 12))
        .sum()
}

fn parse_digits(line: &str) -> Vec<u32> {
    line.chars()
        .map(|d| d.to_digit(10).expect("Not a digit!"))
        .collect::<Vec<u32>>()
}

fn calculate_battery_value(digits: &[u32], battery_length: usize) -> u64 {
    let mut largest = vec![0u32; battery_length];

    digits.iter().enumerate().for_each(|(idx, digit)| {
        for i in 0..battery_length {
            if battery_length - i + idx <= digits.len() && *digit > largest[i] {
                largest.iter_mut().skip(i).for_each(|d| *d = 0);
                largest[i] = *digit;
                break;
            }
        }
    });

    let mut final_val: u64 = 0;
    for value in largest.iter() {
        final_val = final_val * 10 + *value as u64;
    }
    final_val
}

#[cfg(test)]
mod test_day3 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/03");
    const INPUT: &str = include_str!("../../data/input/03");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 16927);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 3121910778619);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 167384358365132);
    }
}

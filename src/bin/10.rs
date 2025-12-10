use std::{default, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/example/10").expect("File not found");
    println!("{}", part1(&input));
    // println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let map = parse_input(input);

    let mut largest: u64 = 0;
    for (i, &point_a) in map.iter().enumerate() {
        for &point_b in map.iter().skip(i + 1) {
            let area = (point_a.0.abs_diff(point_b.0) + 1) * (point_a.1.abs_diff(point_b.1) + 1);
            if largest < area {
                largest = area;
            }
        }
    }

    largest
}

fn part2(input: &str) -> u64 {
    todo!()
}

#[derive(Clone, Default)]
struct MachineInput {
    lights: Vec<bool>,
    buttons: Vec<u8>,
    joltages: Vec<u8>,
}

fn parse_input(input: &str) -> Vec<MachineInput> {
    let line_count = input.lines().count();
    let mut machines = Vec::new();
    for line in input.lines() {
        let mut machine = MachineInput::default();
        for (i, token) in line.split_whitespace().enumerate() {
            if i == 0 {
                machine.lights = parse_lights(token)
            } else if i == line_count - 1 {
                machine.buttons = parse_joltages(token)
            } else {
                machine.joltages = parse_button(token)
            }
        }
        machines.push(machine);
    }
    machines
}

fn parse_lights(token: &str) -> Vec<bool> {
    token
        .chars()
        .filter(|&c| c == '.' || c == '#')
        .map(|c| c == '#')
        .collect()
}

fn parse_button(token: &str) -> Vec<u8> {
    token
        .split(',')
        .map(|s| s.chars().filter(|&c| c.is_digit(10)).collect::<String>())
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

fn parse_joltages(token: &str) -> Vec<u8> {
    todo!()
}

#[cfg(test)]
mod test_day10 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/10");
    const INPUT: &str = include_str!("../../data/input/10");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 50);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 4755278336);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 24);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 1534043700);
    }
}

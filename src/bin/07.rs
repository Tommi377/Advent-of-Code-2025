use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("data/input/07").expect("File not found");
    // println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut count = 0u64;
    let mut streams = HashSet::new();
    streams.insert(
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .unwrap(),
    );

    input.lines().for_each(|line| {
        let line_buf = line.chars().collect::<Vec<char>>();
        let mut new_stream = HashSet::new();
        for i in &streams {
            if line_buf[*i] == '^' {
                count += 1;
                if *i > 0 {
                    new_stream.insert(*i - 1);
                }
                if *i < line_buf.len() - 1 {
                    new_stream.insert(*i + 1);
                }
            } else {
                new_stream.insert(*i);
            }
        }
        streams = new_stream;
    });

    count
}

fn part2(input: &str) -> u64 {
    let start_i: usize = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();
    let mut streams = vec![start_i];

    input.lines().enumerate().for_each(|(ln, line)| {
        println!("Line {} ({}): {}", ln, streams.len(), line);
        let line_buf = line.chars().collect::<Vec<char>>();
        let mut new_stream = Vec::new();
        for i in &streams {
            if line_buf[*i] == '^' {
                if *i > 0 {
                    new_stream.push(*i - 1);
                }
                if *i < line_buf.len() - 1 {
                    new_stream.push(*i + 1);
                }
            } else {
                new_stream.push(*i);
            }
        }
        streams = new_stream;
    });

    streams.len() as u64
}

#[cfg(test)]
mod test_day7 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/07");
    const INPUT: &str = include_str!("../../data/input/07");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 1681);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 40);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 7996218225744);
    }
}

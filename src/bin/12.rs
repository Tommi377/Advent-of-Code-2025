use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("data/input/12").expect("File not found");
    println!("{}", part1(&input));
    // println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let (presents, spots) = parse_input(input);
    spots
        .into_iter()
        .filter(|&spot| {
            let area = spot.width as u64 * spot.height as u64;
            let max_present_area = spot
                .counts
                .iter()
                .enumerate()
                .map(|(i, &count)| {
                    let present = presents.get(&(i as u8)).unwrap();
                    let present_area =
                        present.shape.iter().flatten().filter(|&&b| b).count() as u64;
                    count as u64 * present_area
                })
                .sum::<u64>();
            area > max_present_area
        })
        .count() as u64
}

fn parse_input(input: &str) -> (HashMap<u8, Present>, Vec<Spot>) {
    let mut map = HashMap::<u8, Present>::new();
    let mut lines_iter = input.lines();
    for i in 0..6 {
        lines_iter.next();

        let mut present = Present::default();
        for y in 0..3 {
            let line = lines_iter.next().unwrap();
            for (x, c) in line.chars().enumerate().take(3) {
                match c {
                    '#' => present.shape[y][x] = true,
                    '.' => present.shape[y][x] = false,
                    _ => panic!("Invalid character in present shape"),
                }
            }
        }
        lines_iter.next();

        map.insert(i as u8, present);
    }

    let spots = lines_iter
        .map(|line| {
            let (size, counts) = line.split_once(':').unwrap();
            let (width, height) = size.split_once('x').unwrap();
            Spot {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                counts: counts
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap(),
            }
        })
        .collect::<Vec<Spot>>();

    (map, spots)
}

#[derive(Clone, Copy, Default, Debug)]
struct Present {
    shape: [[bool; 3]; 3],
}

#[derive(Clone, Copy, Default, Debug)]
struct Spot {
    width: u8,
    height: u8,
    counts: [u8; 6],
}

#[cfg(test)]
mod test_day12 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/12");
    const INPUT: &str = include_str!("../../data/input/12");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 509);
    }
}

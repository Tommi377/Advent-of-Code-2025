use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/input/09").expect("File not found");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Point = (u64, u64);

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
    let map = parse_input(input);
    let lines = map
        .clone()
        .into_iter()
        .circular_tuple_windows()
        .collect::<Vec<(Point, Point)>>()
        .clone();

    let mut largest: u64 = 0;
    for (i, &point_a) in map.iter().enumerate() {
        for &point_b in map.iter().skip(i + 1) {
            let (left, right, top, bottom) = bounding_box(point_a, point_b);

            if lines
                .iter()
                .any(|&(a, b)| is_crossing(top, bottom, left, right, a, b))
            {
                continue;
            }

            let area = (point_a.0.abs_diff(point_b.0) + 1) * (point_a.1.abs_diff(point_b.1) + 1);
            if largest < area {
                largest = area;
            }
        }
    }

    largest
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<u64>().expect("Not a number"))
                .collect::<Vec<_>>()
        })
        .map(|nums| (nums[0], nums[1]))
        .collect::<Vec<Point>>()
}

fn is_crossing(
    top: u64,
    bottom: u64,
    left: u64,
    right: u64,
    point_a: Point,
    point_b: Point,
) -> bool {
    let (l, r, t, b) = bounding_box(point_a, point_b);

    if point_a.0 == point_b.0 {
        // Vertical line
        if point_a.0 > left && point_a.0 < right {
            return !(b <= top || t >= bottom);
        }
    } else if point_a.1 == point_b.1 {
        // Horizontal line
        if point_a.1 > top && point_a.1 < bottom {
            return !(r <= left || l >= right);
        }
    }
    false
}

fn bounding_box(point_a: Point, point_b: Point) -> (u64, u64, u64, u64) {
    let hor = if point_a.0 < point_b.0 {
        (point_a.0, point_b.0)
    } else {
        (point_b.0, point_a.0)
    };
    let ver = if point_a.1 < point_b.1 {
        (point_a.1, point_b.1)
    } else {
        (point_b.1, point_a.1)
    };
    (hor.0, hor.1, ver.0, ver.1)
}

#[cfg(test)]
mod test_day9 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/09");
    const INPUT: &str = include_str!("../../data/input/09");

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

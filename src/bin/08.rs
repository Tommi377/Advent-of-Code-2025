use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("data/input/08").expect("File not found");
    println!("{}", part1(&input, 1000));
    println!("{}", part2(&input));
}

type Point = (u64, u64, u64);

fn part1(input: &str, iterations: u64) -> u64 {
    let mut free_id: u64 = 0;
    let mut box_map = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<u64>().expect("Not a number"))
                .collect::<Vec<_>>()
        })
        .map(|nums| ((nums[0], nums[1], nums[2]), None))
        .collect::<HashMap<Point, Option<u64>>>();
    let mut distances: Vec<(u64, (Point, Point))> = Vec::new();
    let mut counts: Vec<u64> = Vec::new();

    for (i, box_a) in box_map.keys().enumerate() {
        for box_b in box_map.keys().skip(i + 1) {
            distances.push((get_distance(*box_a, *box_b), (*box_a, *box_b)));
        }
    }

    distances.sort_unstable();

    for _ in 0..iterations {
        let (_, (box_a, box_b)) = distances.remove(0);

        match (
            box_map.get(&box_a).and_then(|&d| d),
            box_map.get(&box_b).and_then(|&d| d),
        ) {
            (Some(a), Some(b)) if a == b => {
                continue;
            }
            (Some(a), Some(b)) => {
                let (small, large) = if a < b { (a, b) } else { (b, a) };
                for (_, v) in box_map.iter_mut() {
                    let Some(g) = v else { continue };
                    if *g == large {
                        *v = Some(small);
                    }
                }
                counts[small as usize] += counts[large as usize];
                counts[large as usize] = 1;
            }
            (Some(value), None) | (None, Some(value)) => {
                box_map.insert(box_a, Some(value));
                box_map.insert(box_b, Some(value));
                counts[value as usize] += 1;
            }
            (None, None) => {
                box_map.insert(box_a, Some(free_id));
                box_map.insert(box_b, Some(free_id));
                if (free_id as usize) < counts.len() {
                    counts[free_id as usize] += 1;
                } else {
                    counts.push(2);
                }
                free_id += 1;
            }
        }
    }

    counts.sort_unstable();
    counts.iter().rev().take(3).product::<u64>()
}

fn part2(input: &str) -> u64 {
    let mut free_id: u64 = 0;
    let mut box_map = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<u64>().expect("Not a number"))
                .collect::<Vec<_>>()
        })
        .map(|nums| ((nums[0], nums[1], nums[2]), None))
        .collect::<HashMap<Point, Option<u64>>>();
    let mut distances: Vec<(u64, (Point, Point))> = Vec::new();
    let mut counts: Vec<u64> = Vec::new();

    for (i, box_a) in box_map.keys().enumerate() {
        for box_b in box_map.keys().skip(i + 1) {
            distances.push((get_distance(*box_a, *box_b), (*box_a, *box_b)));
        }
    }

    distances.sort_unstable();

    let mut res = 0;
    while box_map
        .iter()
        .any(|(_, group)| !group.is_some_and(|x| x == 0))
    {
        let (_, (box_a, box_b)) = distances.remove(0);

        match (
            box_map.get(&box_a).and_then(|&d| d),
            box_map.get(&box_b).and_then(|&d| d),
        ) {
            (Some(a), Some(b)) if a == b => {
                continue;
            }
            (Some(a), Some(b)) => {
                let (small, large) = if a < b { (a, b) } else { (b, a) };
                for (_, v) in box_map.iter_mut() {
                    let Some(g) = v else { continue };
                    if *g == large {
                        *v = Some(small);
                    }
                }
                counts[small as usize] += counts[large as usize];
                counts[large as usize] = 1;
                res = box_a.0 * box_b.0;
            }
            (Some(value), None) | (None, Some(value)) => {
                box_map.insert(box_a, Some(value));
                box_map.insert(box_b, Some(value));
                counts[value as usize] += 1;
                res = box_a.0 * box_b.0;
            }
            (None, None) => {
                box_map.insert(box_a, Some(free_id));
                box_map.insert(box_b, Some(free_id));
                if (free_id as usize) < counts.len() {
                    counts[free_id as usize] += 1;
                } else {
                    counts.push(2);
                }
                free_id += 1;
            }
        }
    }
    res
}

fn get_distance(a: (u64, u64, u64), b: (u64, u64, u64)) -> u64 {
    let x = a.0.abs_diff(b.0);
    let y = a.1.abs_diff(b.1);
    let z = a.2.abs_diff(b.2);
    x * x + y * y + z * z
}

#[cfg(test)]
mod test_day8 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/08");
    const INPUT: &str = include_str!("../../data/input/08");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT, 10), 40);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT, 1000), 46398);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 25272);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 8141888143);
    }
}

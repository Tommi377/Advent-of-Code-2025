use std::fs;

fn main() {
    let input = fs::read_to_string("data/input/04").expect("File not found");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut neighbors = vec![0u32; width * height];
    let mut map = Vec::new();

    input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .for_each(|(idx, c)| {
            if c == '@' {
                let pos = get_pos(width, idx);

                try_increment(&mut neighbors, width, height, pos.0 - 1, pos.1 - 1);
                try_increment(&mut neighbors, width, height, pos.0, pos.1 - 1);
                try_increment(&mut neighbors, width, height, pos.0 + 1, pos.1 - 1);
                try_increment(&mut neighbors, width, height, pos.0 - 1, pos.1);
                try_increment(&mut neighbors, width, height, pos.0 + 1, pos.1);
                try_increment(&mut neighbors, width, height, pos.0 - 1, pos.1 + 1);
                try_increment(&mut neighbors, width, height, pos.0, pos.1 + 1);
                try_increment(&mut neighbors, width, height, pos.0 + 1, pos.1 + 1);
                map.push(idx);
            }
        });

    input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .filter(|(idx, c)| *c == '@' && neighbors[*idx] < 4)
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut neighbors = vec![0u32; width * height];
    let mut map = vec!['.'; width * height];

    input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .for_each(|(idx, c)| {
            if c == '@' {
                let pos = get_pos(width, idx);

                try_increment(&mut neighbors, width, height, pos.0 - 1, pos.1 - 1);
                try_increment(&mut neighbors, width, height, pos.0, pos.1 - 1);
                try_increment(&mut neighbors, width, height, pos.0 + 1, pos.1 - 1);
                try_increment(&mut neighbors, width, height, pos.0 - 1, pos.1);
                try_increment(&mut neighbors, width, height, pos.0 + 1, pos.1);
                try_increment(&mut neighbors, width, height, pos.0 - 1, pos.1 + 1);
                try_increment(&mut neighbors, width, height, pos.0, pos.1 + 1);
                try_increment(&mut neighbors, width, height, pos.0 + 1, pos.1 + 1);
                map[idx] = '@';
            }
        });

    let mut res = 0;
    let mut stop = false;
    while !stop {
        let mut neighbors_clone = neighbors.clone();
        let removed: u32 = map
            .clone()
            .iter_mut()
            .enumerate()
            .map(|(idx, c)| {
                if *c == '@' && neighbors[idx] < 4 {
                    let pos = get_pos(width, idx);
                    map[idx] = '.';

                    try_decrement(&mut neighbors_clone, width, height, pos.0 - 1, pos.1 - 1);
                    try_decrement(&mut neighbors_clone, width, height, pos.0, pos.1 - 1);
                    try_decrement(&mut neighbors_clone, width, height, pos.0 + 1, pos.1 - 1);
                    try_decrement(&mut neighbors_clone, width, height, pos.0 - 1, pos.1);
                    try_decrement(&mut neighbors_clone, width, height, pos.0 + 1, pos.1);
                    try_decrement(&mut neighbors_clone, width, height, pos.0 - 1, pos.1 + 1);
                    try_decrement(&mut neighbors_clone, width, height, pos.0, pos.1 + 1);
                    try_decrement(&mut neighbors_clone, width, height, pos.0 + 1, pos.1 + 1);
                    return 1;
                }
                0
            })
            .sum();
        neighbors = neighbors_clone;
        res += removed;

        if removed == 0 {
            stop = true
        };
    }

    res
}

fn get_pos(width: usize, idx: usize) -> (isize, isize) {
    ((idx % width) as isize, (idx / width) as isize)
}

fn try_increment(map: &mut [u32], width: usize, height: usize, x: isize, y: isize) {
    if x < width as isize && y < height as isize && x >= 0 && y >= 0 {
        let idx = (y as usize) * width + x as usize;
        map[idx] += 1;
    }
}

fn try_decrement(map: &mut [u32], width: usize, height: usize, x: isize, y: isize) {
    if x < width as isize && y < height as isize && x >= 0 && y >= 0 {
        let idx = (y as usize) * width + x as usize;
        map[idx] -= 1;
    }
}

#[cfg(test)]
mod test_day4 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/04");
    const INPUT: &str = include_str!("../../data/input/04");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 1356);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 43);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 8713);
    }
}

use std::{
    collections::{HashMap, HashSet, LinkedList},
    fs,
};

fn main() {
    let example_a = fs::read_to_string("data/example/11a").expect("File not found");
    let example_b = fs::read_to_string("data/example/11b").expect("File not found");
    let input = fs::read_to_string("data/input/11").expect("File not found");
    println!("{}", part1(&example_a));
    println!("{}", part2(&example_b));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    dfs_unique_paths_a(&"you".to_string(), &parse_input(input), &mut HashSet::new())
}

fn part2(input: &str) -> u64 {
    dfs_unique_paths_b(
        &"svr".to_string(),
        &parse_input(input),
        &mut HashSet::new(),
        &mut HashMap::new(),
    )
    .3
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let parts: Vec<&str> = line.split(":").collect();
        let key = parts[0].to_string();
        let values: Vec<String> = parts[1].trim().split(" ").map(|s| s.to_string()).collect();
        acc.insert(key, values);
        acc
    })
}

fn dfs_unique_paths_a(
    node: &String,
    map: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) -> u64 {
    if node == "out" {
        return 1;
    }

    let mut res = 0;
    if let Some(neighbors) = map.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                visited.insert(node.clone());
                res += dfs_unique_paths_a(neighbor, map, visited);
                visited.remove(node);
            }
        }
    }
    res
}
fn dfs_unique_paths_b(
    node: &String,
    map: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    memo: &mut HashMap<String, (u64, u64, u64, u64)>,
) -> (u64, u64, u64, u64) {
    // if node == "out"
    //     && (visited.contains(&String::from("dac")) && visited.contains(&String::from("fft")))
    if node == "out" {
        return (1, 0, 0, 0);
    }

    if memo.contains_key(node) {
        return *memo.get(node).unwrap();
    }

    let mut value = (0, 0, 0, 0);
    if let Some(neighbors) = map.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                visited.insert(node.clone());

                let (a, b, c, d) = dfs_unique_paths_b(neighbor, map, visited, memo);
                value.0 += a;
                value.1 += b;
                value.2 += c;
                value.3 += d;

                visited.remove(node);
            }
        }
    }

    if node == "dac" {
        value.1 += value.0;
        value.3 += value.2;
        value.0 = 0;
        value.2 = 0;
    }

    if node == "fft" {
        value.2 += value.0;
        value.3 += value.1;
        value.0 = 0;
        value.1 = 0;
    }

    memo.insert(node.clone(), value);

    value
}

#[cfg(test)]
mod test_day11 {
    use super::*;

    const EXAMPLE_A_INPUT: &str = include_str!("../../data/example/11a");
    const EXAMPLE_B_INPUT: &str = include_str!("../../data/example/11b");
    const INPUT: &str = include_str!("../../data/input/11");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_A_INPUT), 5);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 758);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_B_INPUT), 33);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 20083);
    }
}

use std::{
    collections::{HashMap, LinkedList},
    fs,
};

use z3::{Optimize, ast::Int};

#[derive(Clone, Default, Debug)]
struct MachineInput {
    lights: Vec<bool>,
    buttons: Vec<Vec<u16>>,
    joltages: Vec<u16>,
}

fn main() {
    let input = fs::read_to_string("data/input/10").expect("File not found");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    parse_input(input).iter().map(solve_light).sum()
}

fn part2(input: &str) -> u64 {
    parse_input(input).iter().map(solve_joltage).sum()
}

fn solve_light(machine: &MachineInput) -> u64 {
    let mut queue: LinkedList<Vec<bool>> = LinkedList::new();
    let mut came_from: HashMap<Vec<bool>, Vec<bool>> = HashMap::new();

    let start_state = vec![false; machine.lights.len()];
    queue.push_back(start_state.clone());
    while let Some(state) = queue.pop_front() {
        for button in &machine.buttons {
            let new_state = apply_button_light(&state, button);

            if new_state == machine.lights {
                let mut path_length = 1;
                let mut current_state = state;
                while let Some(prev_state) = came_from.get(&current_state) {
                    path_length += 1;
                    current_state = prev_state.clone();
                    if current_state == start_state {
                        break;
                    }
                }
                return path_length;
            }

            if came_from.contains_key(&new_state) {
                continue;
            }

            came_from.insert(new_state.clone(), state.clone());
            queue.push_back(new_state);
        }
    }
    unreachable!()
}

fn solve_joltage(machine: &MachineInput) -> u64 {
    let solver = Optimize::new();

    let button_frees = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(button_idx, _)| Int::new_const(format!("{button_idx}")))
        .collect::<Vec<_>>();

    button_frees
        .iter()
        .for_each(|button_free| solver.assert(&button_free.ge(0)));

    for joltage_idx in 0..machine.joltages.len() {
        let sum_expr = machine
            .buttons
            .iter()
            .enumerate()
            .filter(|(_, button)| button.contains(&(joltage_idx as u16)))
            .map(|(button_idx, _)| Int::new_const(format!("{button_idx}")))
            .fold(Int::from_u64(0), |acc, expr| acc + expr);

        let value = machine.joltages[joltage_idx];
        solver.assert(&sum_expr.eq(value));
    }

    solver.minimize(
        &button_frees
            .iter()
            .fold(Int::from_u64(0), |acc, expr| acc + expr),
    );
    solver.check(&[]);
    let model = solver.get_model().unwrap();
    machine
        .buttons
        .iter()
        .enumerate()
        .map(|(i, _)| {
            model
                .eval(&button_frees[i], true)
                .unwrap()
                .as_u64()
                .unwrap()
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<MachineInput> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let mut machine = MachineInput::default();
        let mut it = line.split_whitespace().enumerate().peekable();
        while let Some((i, token)) = it.next() {
            if i == 0 {
                machine.lights = parse_lights(token)
            } else if it.peek().is_none() {
                machine.joltages = parse_comma_separated(token)
            } else {
                machine.buttons.push(parse_comma_separated(token))
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

fn parse_comma_separated(token: &str) -> Vec<u16> {
    token
        .split(',')
        .map(|s| {
            s.chars()
                .filter(|&c| c.is_ascii_digit())
                .map(|c| ((c as u8) - b'0') as u16)
                .fold(0, |acc, d| acc * 10 + d)
        })
        .collect()
}

fn apply_button_light(state: &[bool], button: &Vec<u16>) -> Vec<bool> {
    let mut new_state = Vec::from(state);
    for &index in button {
        if let Some(light) = new_state.get_mut(index as usize) {
            *light = !*light;
        }
    }
    new_state
}

#[cfg(test)]
mod test_day10 {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../data/example/10");
    const INPUT: &str = include_str!("../../data/input/10");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(INPUT), 509);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 33);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT), 20083);
    }
}

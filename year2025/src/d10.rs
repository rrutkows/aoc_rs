use std::collections::{HashSet, VecDeque};

use good_lp::{Expression, ProblemVariables, Solution, SolverModel, constraint};

pub fn solve01(input: &str) -> i32 {
    input
        .lines()
        .map(parse)
        .map(|(e, _, b)| solve_machine01(e, &b))
        .sum()
}

pub fn solve02(input: &str) -> f64 {
    input
        .lines()
        .map(parse)
        .map(|(_, e, b)| solve_machine02(&e, &b))
        .sum()
}

fn solve_machine01(expected: u16, buttons: &[u16]) -> i32 {
    let mut q = VecDeque::from([(0, 0)]); // Q El is (lights, button press count)
    let mut visited: HashSet<u16> = HashSet::new();
    while let Some((lights, press_count)) = q.pop_front() {
        if lights == expected {
            return press_count;
        }

        for b in buttons {
            let next_lights = lights ^ b;
            if visited.insert(next_lights) {
                q.push_back((next_lights, press_count + 1));
            }
        }
    }
    unreachable!();
}

fn solve_machine02(expected: &[u16], buttons: &[u16]) -> f64 {
    let press_count_limit = expected.iter().max().unwrap();
    let mut variables = ProblemVariables::new();
    let x = variables.add_vector(
        good_lp::variable().integer().clamp(0, *press_count_limit),
        buttons.len(),
    );

    let objective: Expression = x.iter().sum();
    let mut problem = variables.minimise(objective).using(good_lp::scip);
    for (i, &joltage) in expected.iter().enumerate() {
        let sum: Expression = x
            .iter()
            .enumerate()
            .filter(|(b_idx, _)| buttons[*b_idx] & (1 << i) != 0)
            .map(|(_, x)| *x)
            .sum();
        problem = problem.with(constraint!(sum == joltage));
    }
    let solution = problem.solve().unwrap();
    x.iter().map(|x| solution.value(*x)).sum()
}

fn parse(line: &str) -> (u16, Vec<u16>, Vec<u16>) {
    let fragments: Vec<&str> = line.split_whitespace().collect();
    let expected_lights = fragments[0]
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| 1 << (i - 1))
        .sum();
    let buttons = fragments[1..fragments.len() - 1]
        .iter()
        .map(|fragment| {
            fragment[1..fragment.len() - 1]
                .split(',')
                .map(|light_idx| light_idx.parse::<usize>().unwrap())
                .map(|light_idx| 1 << light_idx)
                .sum()
        })
        .collect();
    let expected_joltage = fragments[fragments.len() - 1];
    let expected_joltage = expected_joltage[1..expected_joltage.len() - 1]
        .split(',')
        .map(|joltage| joltage.parse().unwrap())
        .collect();

    (expected_lights, expected_joltage, buttons)
}

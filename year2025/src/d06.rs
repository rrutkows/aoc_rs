use std::{
    ops::{Add, Mul},
    str::FromStr,
};

enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply<A, B>(&self, a: A, b: B) -> A
    where
        A: Mul<B, Output = A> + Add<B, Output = A>,
    {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

fn parse_op(s: &str) -> Vec<Op> {
    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub fn solve01(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let operations = parse_op(lines.last().unwrap());
    lines[..lines.len() - 1]
        .iter()
        .map(|line| line.split_whitespace())
        .map(|numbers| {
            numbers
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .reduce(|acc, i| {
            acc.iter()
                .zip(i)
                .enumerate()
                .map(|(idx, (a, b))| operations[idx].apply(*a, b))
                .collect()
        })
        .unwrap()
        .into_iter()
        .sum()
}

pub fn solve02(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let operations = parse_op(lines.last().unwrap());
    let grid: Vec<&[u8]> = lines[..lines.len() - 1]
        .iter()
        .map(|line| line.as_bytes())
        .collect();
    let w = grid[0].len();
    let mut sum = 0_u64;

    let mut idx = 0;
    let mut current: Option<u64> = None;
    for x in 0..w {
        let column: Vec<u8> = grid.iter().map(|row| row[x]).collect();
        let column = String::from_utf8(column).unwrap();
        match column.trim().parse::<u64>() {
            Ok(n) => {
                current = current.map(|c| operations[idx].apply(c, n)).or(Some(n));
            }
            Err(_) => {
                sum += current.unwrap_or(0);
                idx += 1;
                current = None;
            }
        }
    }

    sum + current.unwrap_or(0)
}

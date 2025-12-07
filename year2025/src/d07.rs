use std::collections::{HashMap, HashSet};

pub fn solve01(input: &str) -> u64 {
    let mut splits = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    for row in input.lines().map(str::as_bytes) {
        for (x, b) in row.iter().enumerate() {
            if *b == b'S' {
                beams.insert(x);
            } else if *b == b'^' && beams.contains(&x) {
                beams.remove(&x);
                beams.insert(x - 1);
                beams.insert(x + 1);
                splits += 1;
            }
        }
    }

    splits
}

pub fn solve02(input: &str) -> u64 {
    let mut beams = HashMap::new();
    for row in input.lines().map(str::as_bytes) {
        for (x, b) in row.iter().enumerate() {
            if *b == b'S' {
                beams.insert(x, 1);
            } else if *b == b'^'
                && let Some(&count) = beams.get(&x)
            {
                beams.remove(&x);
                let left = beams.entry(x - 1).or_insert(0);
                *left += count;
                let right = beams.entry(x + 1).or_insert(0);
                *right += count;
            }
        }
    }

    beams.values().sum()
}

use std::{
    array,
    collections::{HashMap, HashSet, hash_map::Entry},
    str::FromStr,
};

struct JBox([u64; 3]);

impl FromStr for JBox {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',').map(|s| s.parse().unwrap());
        Ok(Self(array::from_fn(|_| coords.next().unwrap())))
    }
}

impl JBox {
    fn sq_distance(&self, other: &Self) -> u64 {
        let JBox(coords1) = self;
        let JBox(coords2) = other;
        coords1
            .iter()
            .zip(coords2.iter())
            .map(|(&c1, &c2)| c1.abs_diff(c2).pow(2))
            .sum()
    }
}

pub fn solve(input: &str) -> (usize, u64) {
    let boxes: Vec<JBox> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut distances: Vec<(u64, usize, usize)> = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b1)| {
            boxes
                .iter()
                .skip(i + 1)
                .enumerate()
                .map(move |(j, b2)| (b1.sq_distance(b2), i, j + i + 1))
        })
        .collect();
    distances.sort_unstable_by_key(|d| d.0);

    let mut boxes2circuits = HashMap::new();
    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut p1 = 0;
    for (connection_idx, (_, i1, i2)) in distances.into_iter().enumerate() {
        let c_idx1 = if let Entry::Vacant(entry) = boxes2circuits.entry(i1) {
            let mut circuit = HashSet::new();
            circuit.insert(i1);
            circuits.push(circuit);
            entry.insert(circuits.len() - 1);
            circuits.len() - 1
        } else {
            boxes2circuits[&i1]
        };

        if boxes2circuits.contains_key(&i2) {
            let c_idx2 = boxes2circuits[&i2];
            if c_idx1 != c_idx2 {
                let other: Vec<usize> = circuits[c_idx2]
                    .drain()
                    .inspect(|b| {
                        boxes2circuits.insert(*b, c_idx1);
                    })
                    .collect();
                circuits[c_idx1].extend(other);
            }
        } else {
            boxes2circuits.insert(i2, c_idx1);
            circuits[c_idx1].insert(i2);
        }

        if (connection_idx + 1) == 1000 {
            let mut sorted = circuits.clone();
            sorted.sort_unstable_by_key(|c| c.len());
            p1 = sorted.iter().rev().take(3).map(|c| c.len()).product();
        }

        if circuits[c_idx1].len() == boxes.len() {
            return (p1, boxes[i1].0[0] * boxes[i2].0[0]);
        }
    }

    unreachable!()
}

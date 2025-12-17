use std::{array, str::FromStr};

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

struct Dsu {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    fn find_parent(&mut self, i: usize) -> usize {
        if self.parents[i] == i {
            i
        } else {
            // path compression
            self.parents[i] = self.find_parent(self.parents[i]);
            self.parents[i]
        }
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find_parent(a);
        let mut b = self.find_parent(b);
        if a != b {
            if self.sizes[a] < self.sizes[b] {
                (a, b) = (b, a);
            }

            self.parents[b] = a;
            self.sizes[a] += self.sizes[b];
            true
        } else {
            false
        }
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

    let mut dsu = Dsu::new(boxes.len());

    let mut p1 = 0;
    let mut unions = 0;
    for (connection_idx, (_, i1, i2)) in distances.into_iter().enumerate() {
        if dsu.union(i1, i2) {
            unions += 1
        }
        if (connection_idx + 1) == 1000 {
            let mut sizes: Vec<usize> = dsu
                .sizes
                .iter()
                .enumerate()
                .filter(|(i, _)| dsu.parents[*i] == *i)
                .map(|(_, s)| *s)
                .collect();
            sizes.sort_unstable();
            p1 = sizes.iter().rev().take(3).product();
        }

        if unions + 1 == boxes.len() {
            return (p1, boxes[i1].0[0] * boxes[i2].0[0]);
        }
    }

    unreachable!()
}

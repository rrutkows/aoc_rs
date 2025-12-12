use std::str::FromStr;

#[derive(Debug)]
struct Region {
    w: usize,
    h: usize,
    shape_count: Vec<usize>,
}

impl FromStr for Region {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (wh, shape_count) = s.split_once(": ").unwrap();
        let (w, h) = wh.split_once('x').unwrap();
        let shape_count = shape_count
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Self {
            w: w.parse().unwrap(),
            h: h.parse().unwrap(),
            shape_count,
        })
    }
}

pub fn solve(input: &str) -> usize {
    let (shapes, regions) = parse(input);
    regions
        .filter(|region| {
            let shapes_total_size = region
                .shape_count
                .iter()
                .zip(shapes.iter())
                .map(|(a, b)| a * b)
                .sum();
            region.w * region.h > shapes_total_size
        })
        .count()
}

fn parse(input: &str) -> (Vec<usize>, impl Iterator<Item = Region>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let shapes: Vec<usize> = parts[..parts.len() - 1]
        .iter()
        .map(|part| {
            part.lines()
                .skip(1)
                .flat_map(|line| line.chars().filter(|c| *c == '#'))
                .count()
        })
        .collect();
    let regions = parts
        .last()
        .unwrap()
        .lines()
        .map(|line| line.parse::<Region>().unwrap());
    (shapes, regions)
}

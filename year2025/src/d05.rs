use std::cmp::{max, min};

pub fn solve01(input: &str) -> usize {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(u64, u64)> = parse(ranges).collect();

    ids.lines()
        .map(|id| id.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|&(s, e)| id >= &s && id <= &e))
        .count()
}

pub fn solve02(input: &str) -> u64 {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut merged: Vec<(u64, u64)> = vec![];
    for (mut s, mut e) in parse(ranges) {
        let mut i = 0;
        while i < merged.len() {
            let (m_s, m_e) = merged[i];
            if s <= m_e && e >= m_s {
                merged.swap_remove(i);
                s = min(s, m_s);
                e = max(e, m_e);
            } else {
                i += 1;
            }
        }

        merged.push((s, e));
    }

    merged.into_iter().map(|(s, e)| e - s + 1).sum()
}

fn parse(ranges: &str) -> impl Iterator<Item = (u64, u64)> {
    ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
}

use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn count_adjacent<F>(
    map: &[&[u8]],
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    limit: i32,
    is_removed: F,
) -> i32
where
    F: Fn(&(usize, usize)) -> bool,
{
    let mut adjacent_count = 0;
    for (dx, dy) in DIRECTIONS {
        if let Some(nx) = x.checked_add_signed(dx)
            && let Some(ny) = y.checked_add_signed(dy)
            && nx < w
            && ny < h
            && map[ny][nx] == b'@'
            && !is_removed(&(nx, ny))
        {
            adjacent_count += 1;
            if adjacent_count >= limit {
                break;
            }
        }
    }
    adjacent_count
}

pub fn solve01(input: &str) -> i32 {
    let map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let w = map[0].len();
    let h = map.len();

    let mut sum = 0;
    for y in 0..h {
        for x in 0..w {
            if map[y][x] == b'@' && count_adjacent(&map, x, y, w, h, 4, |_| false) < 4 {
                sum += 1;
            }
        }
    }

    sum
}

pub fn solve02(input: &str) -> i32 {
    let map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let mut removed: HashSet<(usize, usize)> = HashSet::new();
    let w = map[0].len();
    let h = map.len();

    let mut sum = 0;
    loop {
        let mut current_removed: Vec<(usize, usize)> = Vec::new();
        for y in 0..h {
            for x in 0..w {
                if map[y][x] == b'@'
                    && !removed.contains(&(x, y))
                    && count_adjacent(&map, x, y, w, h, 4, |coords| removed.contains(coords)) < 4
                {
                    current_removed.push((x, y));
                }
            }
        }

        if current_removed.is_empty() {
            break;
        }

        for coords in &current_removed {
            removed.insert(*coords);
        }

        sum += current_removed.len() as i32;
    }

    sum
}

use std::cmp::{max, min};

pub fn solve(input: &str) -> (u64, u64) {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let vertical: Vec<(u64, u64, u64)> = coords
        .iter()
        .enumerate()
        .map(|(i, (x1, y1))| {
            let (x2, y2) = coords[(i + 1) % coords.len()];
            (*x1, x2, *y1, y2)
        })
        .filter(|(x1, x2, _, _)| *x1 == *x2)
        .map(|(x1, _, y1, y2)| (x1, y1, y2))
        .collect();
    let mut p1 = u64::MIN;
    let mut p2 = u64::MIN;
    for (area, x1, x2, y1, y2) in coords.iter().enumerate().flat_map(|(i, (x1, y1))| {
        coords.iter().skip(i + 1).map(|(x2, y2)| {
            let area = (1 + x1.abs_diff(*x2)) * (1 + y1.abs_diff(*y2));
            (area, *x1, *x2, *y1, *y2)
        })
    }) {
        if area > p1 {
            p1 = area;
        }

        // No point checking if inside, if the rectangle is not larger.
        if area > p2 && is_inside(x1, x2, y1, y2, &vertical) {
            p2 = area;
        }
    }
    (p1, p2)
}

fn is_inside(x1: u64, x2: u64, y1: u64, y2: u64, vertical: &[(u64, u64, u64)]) -> bool {
    for (vx, vy1, vy2) in vertical {
        // Is the vertival edge at the same height as the rectangle?
        if min(vy1, vy2) < max(&y1, &y2) && max(vy1, vy2) > min(&y1, &y2) {
            let down = vy2 > vy1;

            // If the vertical edge goes down, it must no be left of the right rectangle edge.
            if down && vx < max(&x1, &x2) {
                return false;
            }

            // If the vertical edge goes up, it must not be right of the left rectangle edge.
            if !down && vx > min(&x1, &x2) {
                return false;
            }
        }
    }

    true
}

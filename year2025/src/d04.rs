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

const LIMIT: usize = 4;

struct Grid<'a> {
    grid: Vec<&'a [u8]>,
    w: usize,
    h: usize,
    removed: HashSet<(usize, usize)>,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Self {
        let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
        let w = grid[0].len();
        let h = grid.len();
        Self {
            grid,
            w,
            h,
            removed: HashSet::new(),
        }
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.grid
            .get(y)
            .and_then(|&row| row.get(x))
            .is_some_and(|&v| v == b'@' && !self.removed.contains(&(x, y)))
    }

    fn has_fewer_neighbours(&self, x: usize, y: usize, limit: usize) -> bool {
        let mut count = 0;
        for (dx, dy) in DIRECTIONS {
            if self.is_occupied(x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)) {
                count += 1;
                if count >= limit {
                    return false;
                }
            }
        }

        true
    }

    fn remove(&mut self, x: usize, y: usize) {
        self.removed.insert((x, y));
    }
}

pub fn solve01(input: &str) -> i32 {
    let grid = Grid::new(input);

    let mut sum = 0;
    for y in 0..grid.h {
        for x in 0..grid.w {
            if grid.is_occupied(x, y) && grid.has_fewer_neighbours(x, y, LIMIT) {
                sum += 1;
            }
        }
    }

    sum
}

pub fn solve02(input: &str) -> i32 {
    let mut grid = Grid::new(input);
    let mut q: Vec<(usize, usize)> = vec![];
    for y in 0..grid.h {
        for x in 0..grid.w {
            if grid.is_occupied(x, y) && grid.has_fewer_neighbours(x, y, LIMIT) {
                q.push((x, y));
            }
        }
    }

    let mut sum = 0;
    while let Some((x, y)) = q.pop() {
        if grid.is_occupied(x, y) {
            sum += 1;
            grid.remove(x, y);
            for (dx, dy) in DIRECTIONS {
                let nx = x.wrapping_add_signed(dx);
                let ny = y.wrapping_add_signed(dy);
                if grid.is_occupied(nx, ny) && grid.has_fewer_neighbours(nx, ny, LIMIT) {
                    q.push((nx, ny));
                }
            }
        }
    }

    sum
}

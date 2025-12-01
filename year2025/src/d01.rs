#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

pub fn solve01(input: &str) -> i32 {
    let mut position = 50;
    let mut password = 0;
    for (direction, distance) in input.lines().map(parse) {
        position = match direction {
            Direction::Left => position - distance,
            Direction::Right => position + distance,
        };
        if position % 100 == 0 {
            password += 1;
        }
    }

    password
}

pub fn solve02(input: &str) -> i32 {
    let mut position = 50;
    let mut password = 0;
    for (direction, distance) in input.lines().map(parse) {
        let d = match direction {
            Direction::Left => {
                password += (distance + 100 - position) / 100;
                if position == 0 {
                    password -= 1;
                }
                -distance
            }
            Direction::Right => {
                password += (position + distance) / 100;
                distance
            }
        };
        position = (position + d).rem_euclid(100);
    }

    password
}

fn parse(line: &str) -> (Direction, i32) {
    let mut chars = line.chars();
    let direction = chars.next().unwrap();
    let distance: i32 = String::from_iter(chars).parse().unwrap();
    match direction {
        'L' => (Direction::Left, distance),
        'R' => (Direction::Right, distance),
        _ => panic!("incorrect input"),
    }
}

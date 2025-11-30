use std::str::FromStr;

const MAX_SIZE: i32 = 100000;
const TOTAL_SPACE: i32 = 70000000;
const REQUIRED_FREE_SPACE: i32 = 30000000;

#[derive(Debug)]
enum ParseResult {
    CdRoot,
    CdUp,
    Cd,
    List,
    Dir,
    File((String, i32)),
}

pub fn solve01(input: &str) -> i32 {
    use ParseResult::*;

    let mut sum = 0;
    let mut dir_stack: Vec<i32> = Vec::new();

    for pr in input.lines().map(parse) {
        // dbg!(&pr, dir_sizes.len());
        match pr {
            CdRoot | Cd => dir_stack.push(0),
            CdUp => {
                let size = dir_stack.pop().unwrap();
                increase_last(&mut dir_stack, size);
                if size <= MAX_SIZE {
                    sum += size;
                }
            }
            File((_, size)) => increase_last(&mut dir_stack, size),
            List | Dir => {}
        }
    }

    while let Some(size) = dir_stack.pop() {
        if size <= MAX_SIZE {
            sum += size;
        }

        increase_last(&mut dir_stack, size);
    }

    sum
}

pub fn solve02(input: &str) -> i32 {
    use ParseResult::*;

    let mut dir_stack: Vec<i32> = Vec::new();
    let mut dir_sizes: Vec<i32> = Vec::new();
    let mut sum = 0;
    for pr in input.lines().map(parse) {
        match pr {
            CdRoot | Cd => dir_stack.push(0),
            CdUp => {
                let size = dir_stack.pop().unwrap();
                dir_sizes.push(size);
                increase_last(&mut dir_stack, size);
            }
            File((_, size)) => {
                sum += size;
                increase_last(&mut dir_stack, size)
            }
            List | Dir => {}
        }
    }

    while let Some(size) = dir_stack.pop() {
        dir_sizes.push(size);
        increase_last(&mut dir_stack, size);
    }

    let min_dir_size = sum + REQUIRED_FREE_SPACE - TOTAL_SPACE;
    dir_sizes
        .into_iter()
        .filter(|&size| size >= min_dir_size)
        .min()
        .unwrap()
}

fn increase_last(s: &mut [i32], size: i32) {
    let len = s.len();
    if len > 0 {
        s[len - 1] += size;
    }
}

fn parse(s: &str) -> ParseResult {
    use ParseResult::*;

    match s.split_once(' ').unwrap() {
        ("$", "ls") => List,
        ("$", command) => match command.split_once(' ').unwrap() {
            ("cd", "/") => CdRoot,
            ("cd", "..") => CdUp,
            ("cd", _) => Cd,
            _ => panic!("unknown command"),
        },
        ("dir", _) => Dir,
        (file_size, file_name) => File((file_name.to_owned(), i32::from_str(file_size).unwrap())),
    }
}

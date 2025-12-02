pub fn solve01(input: &str) -> u64 {
    solve(input, is_invalid01)
}

pub fn solve02(input: &str) -> u64 {
    solve(input, is_invalid02)
}

fn solve<F>(input: &str, is_invalid: F) -> u64
where
    F: Fn(&u64) -> bool,
{
    parse(input)
        .flat_map(|(start, end)| start..end + 1)
        .filter(is_invalid)
        .sum()
}

fn is_invalid01(id: &u64) -> bool {
    let id = id.to_string();
    let (x, y) = id.split_at(id.len() / 2);
    x == y
}

fn is_invalid02(id: &u64) -> bool {
    let id = id.to_string();
    let l = id.len();
    (1..(l / 2 + 1))
        .filter(|size| l.is_multiple_of(*size))
        .any(|size| {
            (1..(l / size)).all(|i| {
                let current = &id[i * size..(i + 1) * size];
                let prev = &id[(i - 1) * size..i * size];
                prev == current
            })
        })
}

fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.trim().split(',').map(|pair| {
        let (start, end) = pair.split_once('-').unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    })
}

pub fn solve(input: &str, battery_count: usize) -> u64 {
    input
        .lines()
        .map(|line| get_largest(line, battery_count))
        .sum()
}

#[allow(clippy::mut_range_bound)]
fn get_largest(bank: &str, count: usize) -> u64 {
    let mut batteries: Vec<u8> = vec![0; count];
    let mut start = 0;
    for (i, battery) in batteries.iter_mut().enumerate() {
        for j in start..=(bank.len() + i - count) {
            if bank.as_bytes()[j] > *battery {
                *battery = bank.as_bytes()[j];
                start = j + 1;
            }
        }
    }
    str::from_utf8(&batteries).unwrap().parse().unwrap()
}

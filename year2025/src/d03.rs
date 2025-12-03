pub fn solve(input: &str, battery_count: usize) -> u64 {
    input
        .lines()
        .map(|line| get_largest(line, battery_count))
        .sum()
}

fn get_largest(bank: &str, count: usize) -> u64 {
    let mut batteries: Vec<u8> = vec![0; count];
    let mut start = 0;
    for i in 0..count {
        for j in start..=(bank.len() + i - count) {
            if bank.as_bytes()[j] > batteries[i] {
                batteries[i] = bank.as_bytes()[j];
                start = j + 1;
            }
        }
    }
    str::from_utf8(&batteries).unwrap().parse().unwrap()
}

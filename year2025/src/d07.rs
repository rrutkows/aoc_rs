pub fn solve(input: &str) -> (i32, u64) {
    let w = input.find('\n').unwrap();
    let mut splits = 0;
    let mut beams = vec![0_u64; w];
    for row in input.lines().map(str::as_bytes) {
        for (x, b) in row.iter().enumerate() {
            if *b == b'S' {
                beams[x] = 1;
            } else if *b == b'^' && beams[x] > 0 {
                beams[x - 1] += beams[x];
                beams[x + 1] += beams[x];
                beams[x] = 0;
                splits += 1;
            }
        }
    }

    (splits, beams.into_iter().sum())
}

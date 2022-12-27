type Coords = [u8; 3];

pub fn solve(input: &str) -> usize {
    let coords_list: Vec<Coords> = parse(input).collect();
    coords_list.len() * 6
        - 2 * coords_list
            .iter()
            .enumerate()
            .map(|(i, c1)| {
                coords_list
                    .iter()
                    .skip(i + 1)
                    .filter(|c2| {
                        std::iter::zip(*c1, **c2)
                            .map(|(c1, c2)| c1.abs_diff(c2))
                            .sum::<u8>()
                            == 1
                    })
                    .count()
            })
            .sum::<usize>()
}

fn parse(input: &str) -> impl Iterator<Item = Coords> + '_ {
    input.lines().map(|line| {
        let mut coords = line.split(',');
        std::array::from_fn(|_| coords.next().unwrap().parse().unwrap())
    })
}

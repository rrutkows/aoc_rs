use aoc2022::d09;

fn main() {
    let input = aoc2022::get_input(9);
    aoc2022::run(|| d09::solve(&input, 2));
    aoc2022::run(|| d09::solve(&input, 10));
}

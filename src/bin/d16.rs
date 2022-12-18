use aoc2022::d16;

fn main() {
    let input = aoc2022::get_input(16);
    aoc2022::run(|| d16::solve(&input, 1));
    aoc2022::run(|| d16::solve(&input, 2));
}

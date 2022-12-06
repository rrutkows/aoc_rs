use aoc2022::d06;

fn main() {
    let input = aoc2022::get_input(6);
    aoc2022::run(|| d06::solve(&input, 4));
    aoc2022::run(|| d06::solve(&input, 14));
}

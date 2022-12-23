use aoc2022::d17;

fn main() {
    let input = aoc2022::get_input(17);
    aoc2022::run(|| d17::solve(&input, 2022));
    aoc2022::run(|| d17::solve(&input, 1000000000000));
}

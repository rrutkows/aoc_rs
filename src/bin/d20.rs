use aoc2022::d20;

fn main() {
    let input = aoc2022::get_input(20);
    aoc2022::run(|| d20::solve(&input, 1, 1));
    aoc2022::run(|| d20::solve(&input, 811589153, 10));
}

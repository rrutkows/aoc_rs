use aoc2022::d07;

fn main() {
    let input = aoc2022::get_input(7);
    aoc2022::run(|| d07::solve01(&input));
    aoc2022::run(|| d07::solve02(&input));
}

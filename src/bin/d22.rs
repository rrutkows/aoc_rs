use aoc2022::d22;

fn main() {
    let input = aoc2022::get_input(22);
    aoc2022::run(|| d22::solve01(&input));
    aoc2022::run(|| d22::solve02(&input));
}

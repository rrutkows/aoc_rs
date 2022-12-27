use aoc2022::d19;

fn main() {
    let input = aoc2022::get_input(19);
    aoc2022::run(|| d19::solve01(&input));
    aoc2022::run(|| d19::solve02(&input));
}

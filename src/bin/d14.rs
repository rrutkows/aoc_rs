use aoc2022::d14;

fn main() {
    let input = aoc2022::get_input(14);
    aoc2022::run(|| d14::solve01(&input));
    aoc2022::run(|| d14::solve02(&input));
}

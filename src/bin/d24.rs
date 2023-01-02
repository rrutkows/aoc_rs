use aoc2022::d24;

fn main() {
    let input = aoc2022::get_input(24);
    aoc2022::run(|| d24::solve01(&input));
    aoc2022::run(|| d24::solve02(&input));
}

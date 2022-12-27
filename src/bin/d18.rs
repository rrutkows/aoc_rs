use aoc2022::d18;

fn main() {
    let input = aoc2022::get_input(18);
    aoc2022::run(|| d18::solve01(&input));
    aoc2022::run(|| d18::solve02(&input));
}

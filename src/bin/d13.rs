use aoc2022::d13;

fn main() {
    let input = aoc2022::get_input(13);
    aoc2022::run(|| d13::solve01(&input));
    aoc2022::run(|| d13::solve02(&input));
}

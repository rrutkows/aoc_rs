use aoc2022::d15;

fn main() {
    let input = aoc2022::get_input(15);
    aoc2022::run(|| d15::solve01(&input));
    aoc2022::run(|| d15::solve02(&input));
}

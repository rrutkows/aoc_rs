use aoc2022::d08;

fn main() {
    let input = aoc2022::get_input(8);
    aoc2022::run(|| d08::solve01(&input));
    aoc2022::run(|| d08::solve02(&input));
}

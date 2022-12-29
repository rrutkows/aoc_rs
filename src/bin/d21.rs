use aoc2022::d21;

fn main() {
    let input = aoc2022::get_input(21);
    aoc2022::run(|| d21::solve01(&input));
    aoc2022::run(|| d21::solve02(&input));
}

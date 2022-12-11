use aoc2022::d11;

fn main() {
    let input = aoc2022::get_input(11);
    aoc2022::run(|| d11::solve(&input, 20, 3));
    aoc2022::run(|| d11::solve(&input, 10000, 1));
}

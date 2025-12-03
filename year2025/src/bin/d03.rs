use year2025::d03;

fn main() {
    let input = year2025::get_input(3);
    common::run(|| d03::solve(&input, 2));
    common::run(|| d03::solve(&input, 12));
}

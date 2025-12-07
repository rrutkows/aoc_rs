use year2025::d07;

fn main() {
    let input = year2025::get_input(7);
    common::run_both(|| d07::solve(&input));
}

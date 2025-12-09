use year2025::d09;

fn main() {
    let input = year2025::get_input(9);
    common::run_both(|| d09::solve(&input));
}

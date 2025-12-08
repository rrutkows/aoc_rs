use year2025::d08;

fn main() {
    let input = year2025::get_input(8);
    common::run_both(|| d08::solve(&input));
}

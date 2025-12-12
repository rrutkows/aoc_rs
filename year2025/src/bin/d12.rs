use year2025::d12;

fn main() {
    let input = year2025::get_input(12);
    common::run(|| d12::solve(&input));
}

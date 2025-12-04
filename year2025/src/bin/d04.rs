use year2025::d04;

fn main() {
    let input = year2025::get_input(4);
    common::run(|| d04::solve01(&input));
    common::run(|| d04::solve02(&input));
}

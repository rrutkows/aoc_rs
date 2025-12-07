use year2025::d07;

fn main() {
    let input = year2025::get_input(7);
    common::run(|| d07::solve01(&input));
    common::run(|| d07::solve02(&input));
}

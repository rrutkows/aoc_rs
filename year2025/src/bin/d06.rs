use year2025::d06;

fn main() {
    let input = year2025::get_input(6);
    common::run(|| d06::solve01(&input));
    common::run(|| d06::solve02(&input));
}

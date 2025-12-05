use year2025::d05;

fn main() {
    let input = year2025::get_input(5);
    common::run(|| d05::solve01(&input));
    common::run(|| d05::solve02(&input));
}

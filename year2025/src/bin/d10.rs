use year2025::d10;

fn main() {
    let input = year2025::get_input(10);
    common::run(|| d10::solve01(&input));
    common::run(|| d10::solve02(&input));
}

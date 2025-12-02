use year2025::d02;

fn main() {
    let input = year2025::get_input(2);
    common::run(|| d02::solve01(&input));
    common::run(|| d02::solve02(&input));
}

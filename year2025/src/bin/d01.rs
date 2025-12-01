use year2025::d01;

fn main() {
    let input = year2025::get_input(1);
    common::run(|| d01::solve01(&input));
    common::run(|| d01::solve02(&input));
}

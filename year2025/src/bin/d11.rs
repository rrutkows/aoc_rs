use year2025::d11;

fn main() {
    let input = year2025::get_input(11);
    common::run(|| d11::solve01(&input));
    common::run(|| d11::solve02(&input));
}

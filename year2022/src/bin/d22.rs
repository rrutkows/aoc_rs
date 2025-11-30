use year2022::d22;

fn main() {
    let input = year2022::get_input(22);
    common::run(|| d22::solve01(&input));
    common::run(|| d22::solve02(&input));
}

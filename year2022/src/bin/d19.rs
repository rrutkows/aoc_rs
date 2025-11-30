use year2022::d19;

fn main() {
    let input = year2022::get_input(19);
    common::run(|| d19::solve01(&input));
    common::run(|| d19::solve02(&input));
}

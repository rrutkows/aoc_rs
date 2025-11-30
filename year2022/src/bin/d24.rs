use year2022::d24;

fn main() {
    let input = year2022::get_input(24);
    common::run(|| d24::solve01(&input));
    common::run(|| d24::solve02(&input));
}

use year2022::d14;

fn main() {
    let input = year2022::get_input(14);
    common::run(|| d14::solve01(&input));
    common::run(|| d14::solve02(&input));
}

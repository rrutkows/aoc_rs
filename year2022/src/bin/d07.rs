use year2022::d07;

fn main() {
    let input = year2022::get_input(7);
    common::run(|| d07::solve01(&input));
    common::run(|| d07::solve02(&input));
}

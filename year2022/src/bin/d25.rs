use year2022::d25;

fn main() {
    let input = year2022::get_input(25);
    common::run(|| d25::solve01(&input));
}

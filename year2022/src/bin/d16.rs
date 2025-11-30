use year2022::d16;

fn main() {
    let input = year2022::get_input(16);
    common::run(|| d16::solve(&input, 1));
    common::run(|| d16::solve(&input, 2));
}

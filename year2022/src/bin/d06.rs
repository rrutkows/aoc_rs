use year2022::d06;

fn main() {
    let input = year2022::get_input(6);
    common::run(|| d06::solve(&input, 4));
    common::run(|| d06::solve(&input, 14));
}

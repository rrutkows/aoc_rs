use year2022::d09;

fn main() {
    let input = year2022::get_input(9);
    common::run(|| d09::solve(&input, 2));
    common::run(|| d09::solve(&input, 10));
}

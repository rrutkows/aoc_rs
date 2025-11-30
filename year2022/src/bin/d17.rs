use year2022::d17;

fn main() {
    let input = year2022::get_input(17);
    common::run(|| d17::solve(&input, 2022));
    common::run(|| d17::solve(&input, 1000000000000));
}

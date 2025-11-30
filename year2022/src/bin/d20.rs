use year2022::d20;

fn main() {
    let input = year2022::get_input(20);
    common::run(|| d20::solve(&input, 1, 1));
    common::run(|| d20::solve(&input, 811589153, 10));
}

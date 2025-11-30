use year2022::d10;

fn main() {
    let input = year2022::get_input(10);
    common::run(|| d10::solve(&input));
}

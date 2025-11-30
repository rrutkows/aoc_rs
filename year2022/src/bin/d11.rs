use year2022::d11;

fn main() {
    let input = year2022::get_input(11);
    common::run(|| d11::solve(&input, 20, 3));
    common::run(|| d11::solve(&input, 10000, 1));
}

use year2022::d15;

fn main() {
    let input = year2022::get_input(15);
    common::run(|| d15::solve01(&input));
    common::run(|| d15::solve02(&input));
}

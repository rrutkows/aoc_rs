use year2022::d13;

fn main() {
    let input = year2022::get_input(13);
    common::run(|| d13::solve01(&input));
    common::run(|| d13::solve02(&input));
}

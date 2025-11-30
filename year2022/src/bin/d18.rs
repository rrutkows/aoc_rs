use year2022::d18;

fn main() {
    let input = year2022::get_input(18);
    common::run(|| d18::solve01(&input));
    common::run(|| d18::solve02(&input));
}

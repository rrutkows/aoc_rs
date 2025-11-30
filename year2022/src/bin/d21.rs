use year2022::d21;

fn main() {
    let input = year2022::get_input(21);
    common::run(|| d21::solve01(&input));
    common::run(|| d21::solve02(&input));
}

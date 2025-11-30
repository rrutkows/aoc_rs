use year2022::d08;

fn main() {
    let input = year2022::get_input(8);
    common::run(|| d08::solve01(&input));
    common::run(|| d08::solve02(&input));
}

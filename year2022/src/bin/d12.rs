use year2022::d12;

fn main() {
    let input = year2022::get_input(12);
    common::run(|| d12::solve01(&input));
    common::run(|| d12::solve02(&input));
}

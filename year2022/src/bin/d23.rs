use year2022::d23;

fn main() {
    let input = year2022::get_input(23);
    common::run(|| d23::solve01(&input));
    common::run(|| d23::solve02(&input));
}

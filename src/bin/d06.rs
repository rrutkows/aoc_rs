use aoc2022::d06;

fn main() {
    let input = aoc2022::get_input(6);
    println!("{}", d06::solve(&input, 4));
    println!("{}", d06::solve(&input, 14));
}

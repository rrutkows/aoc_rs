use aoc2022::d01;

fn main() {
    let input = aoc2022::get_input();
    println!("{}", d01::run(&input, 1));
    println!("{}", d01::run(&input, 3));
}

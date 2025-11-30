use year2022::d01;

fn main() {
    let input = year2022::get_input(1);
    println!("{}", d01::run(&input, 1));
    println!("{}", d01::run(&input, 3));
}

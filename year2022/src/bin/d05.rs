use year2022::d05;

fn main() {
    let input = year2022::get_input(5);
    println!("{}", d05::run(&input, true));
    println!("{}", d05::run(&input, false));
}

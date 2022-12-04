pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;

pub fn get_input(day: u8) -> String {
    let path = format!("src/d{day:02}/input.txt");
    std::fs::read_to_string(path).expect("input.txt should be present in src/dnn/")
}

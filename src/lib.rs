pub mod d01;

pub fn get_input() -> String {
    std::fs::read_to_string("src/d01/input.txt").expect("input.txt should be present in src/dnn/")
}

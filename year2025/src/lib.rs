pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;

pub fn get_input(day: u8) -> String {
    common::get_input(env!("CARGO_MANIFEST_DIR"), day)
}

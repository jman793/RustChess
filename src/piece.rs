use crate::{square, pieceType, squareName};
use sdl2::rect::Point;

#[derive(Copy, Clone)]
pub(crate) enum Color {
    WHITE,
    BLACK,
}



pub(crate) struct Piece {
    square: squareName::SquareName,
    color: Color,
    piece_type: pieceType::PieceType,
    is_taken: bool,
    pub(crate) position: Point
}

pub(crate) fn get_pieces(squares: &Vec<square::Square>, is_white: bool) -> Vec<Piece> {
    let range = if is_white {0..16} else {47..63};
    range
    .map(|i| Piece {
        square: squares[i].square_name,
        color: if is_white {Color::WHITE} else {Color::BLACK},
        piece_type: get_piece_type(i),
        is_taken: false,
        position: squares[i].position      
    })
    .collect()
}

//This function is probably not right but we shall see
fn get_piece_type(i: usize) -> pieceType::PieceType {
    let updated_i: usize = if {55..63}.contains(&i) {i-47} else {i}; 
    match updated_i {
        0 => pieceType::PieceType::ROOK,
        1 => pieceType::PieceType::KNIGHT,
        2 => pieceType::PieceType::BISHOP,
        3 => pieceType::PieceType::QUEEN,
        4 => pieceType::PieceType::KING,
        5 => pieceType::PieceType::BISHOP,
        7 => pieceType::PieceType::ROOK,
        _ => pieceType::PieceType::PAWN 
    }
}

impl Piece {
    pub fn get_filename(&self) -> String {

        let color_delimeter = if matches!(self.color, Color::WHITE) {String::from("W")} else {String::from("B")};
        let folder: String = String::from("assets/");
        const SEPARATER: &str = "_";
        let piece_string = self.piece_type.to_string();
        const FILE_EXT: &str = ".png";

        folder + &color_delimeter + SEPARATER + &piece_string + FILE_EXT
    }
}
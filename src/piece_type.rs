use std::fmt;

use crate::{square_name::SquareName, color::Color, piece::Piece};

// Implement your game rules on these enums instead of your other structs
// That way your rendering logic is strict to the driver code and not to the user of your crate
#[derive(Clone, Copy)]
pub enum PieceType {
    KNIGHT,
    QUEEN,
    PAWN,
    BISHOP,
    ROOK,
    KING,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            PieceType::KNIGHT => "Knight",
            PieceType::QUEEN => "Queen",
            PieceType::PAWN => "Pawn",
            PieceType::BISHOP => "Bishop",
            PieceType::ROOK => "Rook",
            PieceType::KING => "King",
        };
        write!(f, "{}", printable)
    }
}

impl PieceType {
    pub fn get_pawn_moves(piece: &Piece) -> Vec<SquareName> {
        let mut possible_moves: Vec<SquareName> = Vec::new();

        match piece.color {
            Color::WHITE => {
                if piece.square_name.rank != 8 {
                    possible_moves.push(SquareName { file: piece.square_name.file, rank: piece.square_name.rank-1 });
                    let diagonal_up_right_move: Option<SquareName> = piece.square_name.diagonal_up_right();
                    let diagonal_up_left_move: Option<SquareName> = piece.square_name.diagonal_up_left();
                    if diagonal_up_right_move.is_some() {
                        possible_moves.push(diagonal_up_right_move.unwrap());
                    }
                    if diagonal_up_left_move.is_some() {
                        possible_moves.push(diagonal_up_left_move.unwrap());
                    }
                }
            }
            Color::BLACK => {
                if piece.square_name.rank != 1 {
                    possible_moves.push(SquareName { file: piece.square_name.file, rank: piece.square_name.rank+1 });
                    let diagonal_down_right_move: Option<SquareName> = piece.square_name.diagonal_down_right();
                    let diagonal_down_left_move: Option<SquareName> = piece.square_name.diagonal_down_left();
                    if diagonal_down_right_move.is_some() {
                        possible_moves.push(diagonal_down_right_move.unwrap());
                    }
                    if diagonal_down_left_move.is_some() {
                        possible_moves.push(diagonal_down_left_move.unwrap());
                    }
                }
            }
        }
        
        return possible_moves;
    }
}

use crate::{color::Color, piece_type, square_name};

#[derive(Copy, Clone)]
pub struct Piece {
    pub square_name: square_name::SquareName,
    color: Color,
    piece_type: piece_type::PieceType,
}

// TODO write some tests for this
fn get_piece_type(i: i32, color: Color) -> piece_type::PieceType {
    match color {
        Color::WHITE => match i {
            8 => piece_type::PieceType::ROOK,
            9 => piece_type::PieceType::KNIGHT,
            10 => piece_type::PieceType::BISHOP,
            11 => piece_type::PieceType::QUEEN,
            12 => piece_type::PieceType::KING,
            13 => piece_type::PieceType::BISHOP,
            14 => piece_type::PieceType::KNIGHT,
            15 => piece_type::PieceType::ROOK,
            _ => piece_type::PieceType::PAWN,
        },
        Color::BLACK => match i {
            0 => piece_type::PieceType::ROOK,
            1 => piece_type::PieceType::KNIGHT,
            2 => piece_type::PieceType::BISHOP,
            3 => piece_type::PieceType::QUEEN,
            4 => piece_type::PieceType::KING,
            5 => piece_type::PieceType::BISHOP,
            6 => piece_type::PieceType::KNIGHT,
            7 => piece_type::PieceType::ROOK,
            _ => piece_type::PieceType::PAWN,
        },
    }
}

impl Piece {
    // TODO write some tests for this
    pub fn get_filename(&self) -> String {
        let color_delimeter = if matches!(self.color, Color::WHITE) {
            String::from("W")
        } else {
            String::from("B")
        };
        let folder: String = String::from("assets/");
        const SEPARATER: &str = "_";
        let piece_string = self.piece_type.to_string();
        const FILE_EXT: &str = ".png";

        folder + &color_delimeter + SEPARATER + &piece_string + FILE_EXT
    }

    // TODO write some tests for this
    pub(crate) fn init_pieces(color: Color) -> Vec<Piece> {
        (0..16)
            .map(|i| Piece {
                square_name: match color {
                    Color::WHITE => square_name::SquareName::new(i + 48),
                    Color::BLACK => square_name::SquareName::new(i),
                },
                color: color,
                piece_type: get_piece_type(i, color),
            })
            .collect()
    }
}

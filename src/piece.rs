
use crate::{color::Color, piece_type::PieceType, square_name::{self, SquareName}, game_state::GameState};

#[derive(Copy, Clone)]
pub struct Piece {
    pub square_name: square_name::SquareName,
    pub color: Color,
    piece_type: PieceType,
}

// TODO write some tests for this
fn get_piece_type(i: i32, color: Color) -> PieceType {
    match color {
        Color::WHITE => match i {
            8 => PieceType::ROOK,
            9 => PieceType::KNIGHT,
            10 => PieceType::BISHOP,
            11 => PieceType::QUEEN,
            12 => PieceType::KING,
            13 => PieceType::BISHOP,
            14 => PieceType::KNIGHT,
            15 => PieceType::ROOK,
            _ => PieceType::PAWN,
        },
        Color::BLACK => match i {
            0 => PieceType::ROOK,
            1 => PieceType::KNIGHT,
            2 => PieceType::BISHOP,
            3 => PieceType::QUEEN,
            4 => PieceType::KING,
            5 => PieceType::BISHOP,
            6 => PieceType::KNIGHT,
            7 => PieceType::ROOK,
            _ => PieceType::PAWN,
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

    // I am not in love with the fact that this function touches the game state to get this done
    // This could be refactored to probably be much cleaner
    // Maybe make the various piece types a "trait" so that getting possible moves is one method call
    pub fn get_legal_moves(&self, game_state: &GameState) -> Vec<SquareName> {
        let mut legal_moves: Vec<SquareName> = Vec::new();
        let mut possible_moves: Vec<SquareName>;

        match self.piece_type {
            PieceType::KNIGHT => todo!(),
            PieceType::QUEEN => todo!(),
            PieceType::PAWN => {
                possible_moves = PieceType::get_pawn_moves(self);

                for piece_move in possible_moves.iter() {
                    if piece_move.file == self.square_name.file {
                        if !game_state.squares[piece_move].is_occupied {
                            legal_moves.push(*piece_move);
                        }
                    }
                    else {
                        if game_state.is_square_occupied_by_opposite_color(*piece_move) {
                            legal_moves.push(*piece_move);
                        }
                    }
                }
            },
            PieceType::BISHOP => todo!(),
            PieceType::ROOK => todo!(),
            PieceType::KING => todo!(),
        }





        

        return legal_moves;
    }
}

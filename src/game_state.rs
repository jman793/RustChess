use std::collections::HashMap;

use crate::{
    color::{self, Color},
    piece::{self, Piece},
    square::Square,
    square_name::SquareName,
};

pub struct GameState {
    pub move_color: color::Color,
    pub squares: HashMap<SquareName, Square>,
    pub white_pieces: Vec<piece::Piece>,
    pub black_pieces: Vec<piece::Piece>,
    pub is_square_selected: bool,
    selected_piece_index: usize,
}

impl GameState {
    pub fn init_game() -> GameState {
        GameState {
            move_color: Color::WHITE,
            squares: Square::initialize_squares(),
            white_pieces: Piece::init_pieces(Color::WHITE),
            black_pieces: Piece::init_pieces(Color::BLACK),
            is_square_selected: false,
            selected_piece_index: usize::MAX,
        }
    }

    pub fn select_square(&mut self, square_name: SquareName) {
        if self.is_occupied_by_correct_color(square_name) {
            self.is_square_selected = true;
            self.selected_piece_index = self.find_piece_on_square(square_name);
        }
    }

    pub fn make_move(&mut self, square_name: SquareName) {
        //(is valid move check here be sure to check if the piece is moving at all :-D )

        if self.is_move_legal(square_name){
            self.move_piece(square_name);
        }
    }

    pub fn is_move_legal(&self, square_name: SquareName) -> bool {
        let legal_moves: Vec<SquareName>;

        match self.move_color {
            Color::WHITE => {
                let piece = self.white_pieces[self.selected_piece_index];
                legal_moves = piece.get_legal_moves(self);
            }
            Color::BLACK => {
                let piece: Piece = self.black_pieces[self.selected_piece_index];
                legal_moves = piece.get_legal_moves(self); 
            }
        }
        println!("{}", legal_moves.contains(&square_name));
        return legal_moves.contains(&square_name);
    }

    pub fn is_square_occupied_by_opposite_color(&self, square_name: SquareName) -> bool {
        if !self.squares[&square_name].is_occupied {
            return false;
        }

        match self.move_color {
            Color::WHITE => {
                for black_piece in self.black_pieces.iter() {
                    if black_piece.square_name == square_name {
                        return true;
                    }
                }
            }
            Color::BLACK => {
                for white_piece in self.white_pieces.iter() {
                    if white_piece.square_name == square_name {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn move_piece(&mut self, square_name: SquareName) {
        match self.move_color {
            color::Color::WHITE => {
                self.squares
                    .get_mut(&self.white_pieces[self.selected_piece_index].square_name)
                    .unwrap()
                    .is_occupied = false;
                self.white_pieces[self.selected_piece_index].square_name = square_name;
                self.move_color = Color::BLACK;
            }
            color::Color::BLACK => {
                self.squares
                    .get_mut(&self.black_pieces[self.selected_piece_index].square_name)
                    .unwrap()
                    .is_occupied = false;
                self.black_pieces[self.selected_piece_index].square_name = square_name;
                self.move_color = Color::WHITE;
            }
        }
        self.squares.get_mut(&square_name).unwrap().is_occupied = true;
        self.is_square_selected = false;
    }

    fn find_piece_on_square(&self, square_name: SquareName) -> usize {
        match self.move_color {
            Color::WHITE => {
                for (index, white_piece) in self.white_pieces.iter().enumerate() {
                    if white_piece.square_name == square_name {
                        return index;
                    }
                }
            }
            Color::BLACK => {
                for (index, black_piece) in self.black_pieces.iter().enumerate() {
                    if black_piece.square_name == square_name {
                        return index;
                    }
                }
            }
        }
        panic!("Piece not found this should not happen");
    }

    fn is_occupied_by_correct_color(&self, square_name: SquareName) -> bool {
        match self.move_color {
            Color::WHITE => {
                for white_piece in self.white_pieces.iter() {
                    if white_piece.square_name == square_name {
                        return true;
                    }
                }
            }
            Color::BLACK => {
                for black_piece in self.black_pieces.iter() {
                    if black_piece.square_name == square_name {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

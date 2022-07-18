use crate::{common, piece, square};

use sdl2::{
    rect::{Point, Rect},
    render::Texture,
    render::WindowCanvas,
};

// TODO write some tests for this
pub(crate) fn render(
    canvas: &mut WindowCanvas,
    textures: &Vec<Texture>,
    white_pieces: &Vec<piece::Piece>,
    black_pieces: &Vec<piece::Piece>,
) -> Result<(), String> {
    canvas.clear();

    let mut rectangles: Vec<sdl2::rect::Rect> = Vec::new();
    rectangles.push(Rect::from_center(
        Point::new(common::WINDOW_SIDE / 2, common::WINDOW_SIDE / 2),
        common::WINDOW_SIDE.try_into().unwrap(),
        common::WINDOW_SIDE.try_into().unwrap(),
    ));

    for piece in white_pieces.iter() {
        rectangles.push(Rect::from_center(
            square::square_name_to_position(piece.square_name),
            100,
            100,
        ));
    }
    for piece in black_pieces.iter() {
        rectangles.push(Rect::from_center(
            square::square_name_to_position(piece.square_name),
            100,
            100,
        ));
    }

    for i in 0..textures.len() {
        canvas.copy(&textures[i], None, rectangles[i])?;
    }

    canvas.present();

    Ok(())
}

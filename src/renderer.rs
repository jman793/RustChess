use crate::{common::constants, game_state};

use sdl2::{
    rect::{Point, Rect},
    render::Texture,
    render::WindowCanvas,
};

// TODO write some tests for this
pub(crate) fn render(
    canvas: &mut WindowCanvas,
    textures: &Vec<Texture>,
    game_state: &game_state::GameState,
) -> Result<(), String> {
    canvas.clear();

    let mut rectangles: Vec<sdl2::rect::Rect> = Vec::new();
    rectangles.push(Rect::from_center(
        Point::new(constants::WINDOW_SIDE / 2, constants::WINDOW_SIDE / 2),
        constants::WINDOW_SIDE.try_into().unwrap(),
        constants::WINDOW_SIDE.try_into().unwrap(),
    ));

    for piece in game_state.white_pieces.iter() {
        rectangles.push(Rect::from_center(
            game_state.squares[&piece.square_name].position,
            100,
            100,
        ));
    }
    for piece in game_state.black_pieces.iter() {
        rectangles.push(Rect::from_center(
            game_state.squares[&piece.square_name].position,
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

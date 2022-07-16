use crate::{piece};

use sdl2::{
    render::Texture,
    render::{WindowCanvas},
    rect::{Rect, Point}
};



pub(crate) fn render(canvas: &mut WindowCanvas, textures: &Vec<Texture>, white_pieces: &Vec<piece::Piece>, black_pieces: &Vec<piece::Piece>) -> Result<(), String> {
    canvas.clear();
    
    let mut rectangles: Vec<sdl2::rect::Rect> = Vec::new();
    rectangles.push(Rect::from_center(Point::new(400,300), 800, 600));


    for piece in white_pieces.iter() {
        rectangles.push(Rect::from_center(piece.position, 50, 40));
    }
    for piece in black_pieces.iter() {
        rectangles.push(Rect::from_center(piece.position, 50, 40));
    }


    for i in 0..textures.len() {
        canvas.copy(&textures[i], None, rectangles[i])?;  
    }
    

    canvas.present();

    Ok(())
}
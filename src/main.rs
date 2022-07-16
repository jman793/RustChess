mod piece;
mod square;
mod common;
mod renderer;
mod squareName;
mod pieceType;

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::{Keycode},
    render::Texture
};

use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let mut squares: Vec<square::Square> = square::initialize_squares();

    let mut white_pieces: Vec<piece::Piece> = piece::get_pieces(&squares, true);
    let mut black_pieces: Vec<piece::Piece> = piece::get_pieces(&squares, false);
    
    let window = video_subsystem
        .window("rust_chess", common::WINDOW_WIDTH, common::WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem"); 

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    
    let texture_creator = canvas.texture_creator();
    let mut textures: Vec<Texture> = Vec::new();
    textures.push(texture_creator.load_texture("assets/board1.png")?);

    for piece in white_pieces.iter() {
        textures.push(texture_creator.load_texture(piece.get_filename())?);
    }
    for piece in black_pieces.iter() {
        textures.push(texture_creator.load_texture(piece.get_filename())?);
    }

    let mut events = sdl_context.event_pump()?;

    // Typical loop goes input(), update(), render()
    'run_loop: loop {
        //input
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'run_loop;
                }
                _ => {}
            }
        }

        //render
        renderer::render(&mut canvas, &textures, &white_pieces, &black_pieces)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

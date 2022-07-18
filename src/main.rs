mod common;
mod piece;
mod piece_type;
mod renderer;
mod square;
mod square_name;

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    mouse::MouseButton,
    render::Texture,
};


fn find_square_from_name(squares: &Vec<square::Square>, square_name: square_name::SquareName) -> Option<square::Square> {
    let mut matched_square: Option<square::Square> = None;
    for square in squares.iter() {
        match square.square_name == square_name {
            true => matched_square = Some(*square),
            false => continue,
        }
    }

    matched_square
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let mut squares: Vec<square::Square> = square::initialize_squares();

    let mut white_pieces: Vec<piece::Piece> = piece::init_pieces(true);
    let mut black_pieces: Vec<piece::Piece> = piece::init_pieces(false);

    let window = video_subsystem
        .window(
            "rust_chess",
            common::WINDOW_SIDE.try_into().unwrap(),
            common::WINDOW_SIDE.try_into().unwrap(),
        )
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
    let mut is_square_selected: bool = false;

    // Typical loop goes input(), update(), render()
    'run_loop: loop {
        //input
        for event in events.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'run_loop;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    if is_square_selected {
                        is_square_selected = false;
                    } else {
                        let selected_square_name = square::select_square_from_position(
                            events.mouse_state().x(),
                            events.mouse_state().y(),
                        );
                        let selected_square: square::Square = find_square_from_name(&squares, selected_square_name)
                        .expect("Could not find square. This should not happen");
                        if selected_square.is_occupied {
                            is_square_selected = true;
                        }
                    }
                }
                _ => {}
            }
        }

        //render
        renderer::render(&mut canvas, &textures, &white_pieces, &black_pieces)?;
    }
    Ok(())
}

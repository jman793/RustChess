mod color;
mod common;
mod game_state;
mod piece;
mod piece_type;
mod renderer;
mod square;
mod square_name;

use common::constants;
use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    mouse::MouseButton,
    render::Texture,
};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let mut game_state: game_state::GameState = game_state::GameState::init_game();

    let window = video_subsystem
        .window(
            "rust_chess",
            constants::WINDOW_SIDE.try_into().unwrap(),
            constants::WINDOW_SIDE.try_into().unwrap(),
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

    for piece in game_state.white_pieces.iter() {
        textures.push(texture_creator.load_texture(piece.get_filename())?);
    }
    for piece in game_state.black_pieces.iter() {
        textures.push(texture_creator.load_texture(piece.get_filename())?);
    }

    // Ideally its time to abstract out your game state,
    // Squares, and both piece vectors as well as turn and move counts
    // This would remove most of these variables below and a lot of logical implementation
    let mut events = sdl_context.event_pump()?;

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
                    if game_state.is_square_selected {
                        let moved_square_name = square::select_square_from_position(
                            events.mouse_state().x(),
                            events.mouse_state().y(),
                        );
                        game_state.make_move(moved_square_name);
                    } else {
                        let selected_square_name = square::select_square_from_position(
                            events.mouse_state().x(),
                            events.mouse_state().y(),
                        );
                        game_state.select_square(selected_square_name);
                    }
                }
                _ => {}
            }
        }

        //render
        renderer::render(&mut canvas, &textures, &game_state)?;
    }
    Ok(())
}

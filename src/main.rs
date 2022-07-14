mod piece;

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    render::Texture,
    render::WindowCanvas,
};

use std::time::Duration;

fn render(canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
    canvas.clear();

    canvas.copy(texture, None, None)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let mut squares: Vec<piece::Square> = piece::initialize_squares();
    
    let window = video_subsystem
        .window("rust_chess", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/chess-board.jpg")?;

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
        render(&mut canvas, &texture)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

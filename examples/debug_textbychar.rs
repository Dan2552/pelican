// This is not really an example of how to use the library, but rather a test
// scenario just to check that rendering character-by-character with SDL TTF
// works nicely.

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl = sdl2::init().unwrap();
    let ttf = sdl2::ttf::init().unwrap();

    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("char by char", 500, 100)
        .position(100, 100)
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    canvas.clear();

    let mut font = ttf.load_font("/System/Library/Fonts/Helvetica.ttc", 16 * 2).unwrap();
    font.set_kerning(false);

    let text = "The quick brown fox jumps over the lazy dog.";
    let surface = font.render(text.clone()).blended(sdl2::pixels::Color::RGB(0, 0, 0)).unwrap();
    let texture = canvas.create_texture_from_surface(&surface).unwrap();
    let destination = sdl2::rect::Rect::new(50, 50, surface.width(), surface.height());
    canvas.copy(&texture, None, destination).unwrap();

    let mut x = 50;
    for char in text.chars() {
        let char_text = char.to_string();
        let surface = font.render(&char_text).blended(sdl2::pixels::Color::RGB(0, 0, 0)).unwrap();
        let texture = canvas.create_texture_from_surface(&surface).unwrap();
        let destination = sdl2::rect::Rect::new(x, 100, surface.width(), surface.height());
        canvas.copy(&texture, None, destination).unwrap();
        x = x + surface.width() as i32;
    }

    canvas.present();

    let mut event_pump = sdl.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

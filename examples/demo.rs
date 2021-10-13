extern crate sdl2;

use pelican::graphics::Point;
use pelican::graphics::Size;
use pelican::graphics::Context;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl = sdl2::init().unwrap();
    
    let position = Point { x: 10, y: 10};
    let size = Size { width: 800, height: 600 };
    let window1 = Context::new(&sdl, "hello world", position, size);

    let position = Point { x: 100, y: 100};
    let size = Size { width: 300, height: 300 };
    let window2 = Context::new(&sdl, "hello world", position, size);

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

    //     canvas.clear();
    //     canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    //     // The rest of the game loop goes here...
    }

    println!("{}", window1.render_scale);
    println!("{}", window2.render_scale);

    Ok(())
}
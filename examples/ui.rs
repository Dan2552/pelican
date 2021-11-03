use pelican::graphics::{Rectangle, Point, Size};
use pelican::ui::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let frame = Rectangle {
        position: Point { x: 10, y: 10 },
        size: Size { width: 50, height: 50 }
    };

    let window = Window::new(frame);

    let sdl: &sdl2::Sdl;
    unsafe { sdl = pelican::graphics::SDL_CONTAINER.lazy(); }


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
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    //     // The rest of the game loop goes here...
    }

    Ok(())
}

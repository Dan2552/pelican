use sdl2::Sdl;
use sdl2::event::Event;
use crate::ui::RunLoop;

pub(crate) fn update(sdl: &Sdl) {
    let mut event_pump = sdl.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                quit();
            }
            _ => {}
        }
    }
}

fn quit() {
    let run_loop = RunLoop::borrow();
    run_loop.exit();
}

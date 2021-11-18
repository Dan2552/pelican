use sdl2::Sdl;
use sdl2::event::Event;
use crate::ui::RunLoop;

// enum EventLoopError {
//     ExitError
// }
// pub(crate) fn update(sdl: &Sdl) -> Result<(), EventLoopError> {
// for event in event_pump.poll_iter() {
//     match event {
//         Event::Quit { .. } => return Err(EventLoopError::ExitError),
//         _ => {}
//     }
// }

pub(crate) fn update(sdl: &Sdl) {
    println!("event_loop::update");
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

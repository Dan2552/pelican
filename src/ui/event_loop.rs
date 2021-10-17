use sdl2::Sdl;
use sdl2::event::Event;

enum EventLoopError {
    ExitError
}

fn update(sdl: &Sdl) -> Result<(), EventLoopError> {
    let mut event_pump = sdl.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return Err(EventLoopError::ExitError),
            _ => {}
        }
    }

    Ok(())
}

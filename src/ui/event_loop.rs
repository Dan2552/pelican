use sdl2::Sdl;
use sdl2::event::Event;

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
            _ => {}
        }
    }

    std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 30));
}

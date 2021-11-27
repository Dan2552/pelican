use sdl2::Sdl;
use crate::ui::{Application, Event};

pub(crate) fn update(sdl: &Sdl) {
    let mut event_pump = sdl.event_pump().unwrap();

    for sdl_event in event_pump.poll_iter() {
        send_event(sdl_event);
    }
}

fn send_event(event: Event) {
    let application = Application::borrow();
    application.send_event(event);
}

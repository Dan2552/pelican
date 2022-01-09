use crate::ui::{Application};
use crate::ui::touch::Touch;
use crate::graphics::Point;
use crate::ui::event::EventArena;

pub(crate) fn update(sdl: &sdl2::Sdl) {
    let mut event_pump = sdl.event_pump().unwrap();
    let mut event_arena = EventArena::borrow_mut();

    event_arena.cleanup_ended_touches();

    for sdl_event in event_pump.poll_iter() {
        match sdl_event {
            sdl2::event::Event::Quit { .. } => {
                let application = Application::borrow();
                application.exit();
            },
            sdl2::event::Event::MouseButtonDown { window_id, x, y, .. } => {
                let touch = Touch::new(
                    0,
                    Point { x, y },
                );

                let application = Application::borrow();
                application.assign_targets_to_touch(window_id, &touch);
                let event = event_arena.touch_began(touch.clone());

                for gesture_recognizer in touch.gesture_recognizers().iter() {
                    if let Some(gesture_recognizer) = gesture_recognizer.upgrade() {
                        gesture_recognizer.touches_began(&event.touches(), &event);
                    }
                }

                if let Some(view) = touch.view() {
                    view.touches_began(&event.touches(), &event);
                }
            },
            sdl2::event::Event::MouseButtonUp { x, y, .. } => {
                event_arena.touch_ended(0, Point { x, y });

                let event = event_arena.touch_event();

                if let Some(existing_touch) = event_arena.touch_event().touches().first() {
                    for gesture_recognizer in existing_touch.gesture_recognizers().iter() {
                        if let Some(gesture_recognizer) = gesture_recognizer.upgrade() {
                            gesture_recognizer.touches_ended(&event.touches(), &event);
                        }
                    }

                    if let Some(view) = existing_touch.view() {
                        view.touches_ended(&event.touches(), &event);
                    }
                }
            },
            sdl2::event::Event::MouseMotion { x, y, .. } => {
                event_arena.touch_moved(0, Point { x, y });

                let event = event_arena.touch_event();

                if let Some(existing_touch) = event_arena.touch_event().touches().first() {
                    for gesture_recognizer in existing_touch.gesture_recognizers().iter() {
                        if let Some(gesture_recognizer) = gesture_recognizer.upgrade() {
                            gesture_recognizer.touches_moved(&event.touches(), &event);
                        }
                    }

                    if let Some(view) = existing_touch.view() {
                        view.touches_moved(&event.touches(), &event);
                    }
                }
            },
            sdl2::event::Event::MultiGesture { .. } => println!("SDL_MultiGestureEvent"),

            // https://stackoverflow.com/a/47597200/869367
            sdl2::event::Event::MouseWheel { window_id, x, y, .. } => {
                let event = event_arena.scroll_event();
                let touch = event.touch();

                let application = Application::borrow();
                application.assign_targets_to_touch(window_id, &touch);

                event_arena.scroll_did_translate(Point::new(x, y));

                for gesture_recognizer in touch.gesture_recognizers().iter() {
                    if let Some(gesture_recognizer) = gesture_recognizer.upgrade() {
                        gesture_recognizer.scroll_did_translate(&event.translation(), &event);
                    }
                }
            },
            _ => (),
        }
    }
}

use crate::graphics::Point;
use crate::ui::input_state::InputState;
use crate::ui::{Window, WeakView, Event, RunLoop};
use crate::singleton;

singleton::singleton!(
    Application, 
    key_window_index: None, 
    windows: Vec::new()
);

pub struct Application {
    key_window_index: Option<usize>,
    pub(crate) windows: Vec<Window>,
}

impl<'a> Application {
    pub(crate) fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn get_key_window(&self) -> WeakView {
        if let Some(index) = self.key_window_index {
            self.windows[index].downgrade()
        } else {
            WeakView::none()
        }
    }

    pub fn set_key_window(&mut self, window: &Window) {
        let position = self.windows.iter().position(|v| v == window).unwrap();
        self.key_window_index = Some(position);
    }

    fn get_window(&self, context_id: u32) -> Option<&Window> {
        for window in self.windows.iter() {
            if window.context_id() == context_id {
                return Some(window);
            }
        }

        None
    }

    pub(crate) fn send_event(&self, event: Event) {
        match event {
            Event::Quit { .. } => quit(),
            Event::MouseButtonDown { .. } => self.mouse_button_down(event),
            Event::MouseButtonUp { .. } => self.mouse_button_up(event),
            Event::MouseMotion { .. } => self.mouse_move(event),
            _  => ()
        }
    }

    fn mouse_button_down(&self, event: Event) {
        match event {
            Event::MouseButtonDown {
                window_id,
                x,
                y,
                ..
            } => {
                let mut input_state = InputState::borrow_mut();

                // Because this is a click and not a real touch, there's no
                // real id, so 0 is always used.
                let touch = input_state.find_or_create_touch(0);
                touch.set_position(Point::new(x, y));

                let window = self.get_window(window_id).unwrap();
                touch.set_window(window.clone());
                if let Some(view) = window.hit_test(&Point { x, y }) {
                    touch.set_view(view.clone());
                    view.touches_began(&vec![touch.clone()], event);
                }
            }
            _ => panic!("unexpected event")
        }
    }

    fn mouse_button_up(&self, event: Event) {
        match event {
            Event::MouseButtonUp {
                x,
                y,
                ..
            } => {
                let mut input_state = InputState::borrow_mut();

                // Because this is a click and not a real touch, there's no
                // real id, so 0 is always used.
                let touch = input_state.find_or_create_touch(0);
                touch.set_position(Point::new(x, y));
                touch.update_timestamp();

                if let Some(view) = touch.get_view() {
                    view.touches_ended(&vec![touch.clone()], event);
                }

                input_state.remove_touch(0);
            }
            _ => panic!("unexpected event")
        }
    }

    fn mouse_move(&self, event: Event) {
        match event {
            Event::MouseMotion {
                x,
                y,
                ..
            } => {
                let mut input_state = InputState::borrow_mut();

                // Because this is a click and not a real touch, there's no
                // real id, so 0 is always used.
                //
                // With "usual" mouse input, you may care about mouse movement
                // even if a click is not currently happening. This is
                // specifically avoided to keep consistent behavior with touch
                // input.
                if let Some(touch) = input_state.find_touch(0) {
                    touch.set_position(Point::new(x, y));
                    touch.update_timestamp();

                    if let Some(view) = touch.get_view() {
                        view.touches_moved(&vec![touch.clone()], event);
                    }
                }
            }
            _ => panic!("unexpected event")
        }
    }
}

fn quit() {
    let run_loop = RunLoop::borrow();
    run_loop.exit();
}

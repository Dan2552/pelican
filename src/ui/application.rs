use crate::graphics::Point;
use crate::ui::{Window, WeakView, Event, RunLoop, Touch};
use crate::singleton;

singleton::singleton!(Application, key_window_index: None, windows: Vec::new());

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
            Event::MouseButtonDown { .. } => self.handle_click(event),
            _  => ()
        }
    }

    fn handle_click(&self, event: Event) {
        // TODO: how to better do this?
        match event {
            Event::MouseButtonDown {
                timestamp,
                window_id,
                mouse_btn,
                x,
                y,
                ..
            } => {
                let touch = Touch::new(0, Point { x, y }, crate::ui::touch::TouchPhase::Began);
                println!("{} clicky {}, {}", window_id, x, y);
                let window = self.get_window(window_id).unwrap();
                if let Some(view) = window.hit_test(&Point { x, y }) {
                    view.touches_began(&vec![touch], event);
                }

            },
            _ => { panic!() }
        }
    }
}

fn quit() {
    let run_loop = RunLoop::borrow();
    run_loop.exit();
}

use crate::ui::{Window, WeakView, RunLoop};
use crate::macros::*;
use crate::ui::touch::Touch;
use crate::ui::gesture::recognizer::Recognizer;
use std::rc::Weak;

singleton!(
    Application,
    key_window_index: None,
    windows: Vec::new()
);

pub struct Application {
    key_window_index: Option<usize>,
    pub(crate) windows: Vec<Window>
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

    pub fn exit(&self) {
        let run_loop = RunLoop::borrow();
        run_loop.exit();
    }

    pub(crate) fn assign_targets_to_touch(&self, window_id: u32, touch: &mut Touch) {
        let window = self.get_window(window_id).unwrap();
        touch.set_window(window.clone());

        if let Some(view) = window.hit_test(&touch.position()) {
            touch.set_view(view.clone());

            let mut recognizers: Vec<Weak<Box<dyn Recognizer>>> = Vec::new();

            for recognizer in view.gesture_recognizers().iter() {
                recognizers.push(recognizer.clone());
            }

            let mut current_view = view;
            while let Some(view) = current_view.superview().upgrade() {
                for recognizer in view.gesture_recognizers().iter() {
                    recognizers.push(recognizer.clone());
                }
                current_view = view;
            }

            touch.set_gesture_recognizers(recognizers);
        }
    }
}

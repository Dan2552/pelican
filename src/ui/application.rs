use crate::ui::View;
use crate::ui::WeakView;
use crate::singleton;

singleton::singleton!(Application, key_window_index: None, windows: Vec::new());

pub struct Application {
    key_window_index: Option<usize>,
    pub(crate) windows: Vec<View>,
}

impl<'a> Application {
    pub(crate) fn add_window(&mut self, view: View) {
        println!("Application: add_window");
        self.windows.push(view);
    }

    pub fn get_key_window(&self) -> WeakView {
        if let Some(index) = self.key_window_index {
            self.windows[index].downgrade()
        } else {
            WeakView::none()
        }
    }

    pub fn set_key_window(&mut self, view: &View) {
        let position = self.windows.iter().position(|v| v == view).unwrap();
        self.key_window_index = Some(position);
    }
}

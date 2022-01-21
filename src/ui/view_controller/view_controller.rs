use crate::ui::View;
use std::cell::Cell;

pub trait ViewControllerBehavior {
    fn view_will_disappear(&self, _view: View) {}
    fn view_did_disappear(&self, _view: View) {}
    fn view_will_appear(&self, _view: View) {}
    fn view_did_appear(&self, _view: View) {}
    fn view_did_load(&self, _view: View) {}
}

#[derive(Copy, Clone)]
enum State {
    WillLoad,
    DidLoad,
    WillAppear,
    DidAppear,
    // WillDisappear,
    // DidDisappear
}

pub struct ViewController<'a> {
    state: Cell<State>,
    behavior: Box<dyn ViewControllerBehavior + 'a>,
}

impl<'a> ViewController<'a> {
    pub fn new<T>(behavior: T) -> Self where T: ViewControllerBehavior + 'a {
        Self {
            state: Cell::new(State::WillLoad),
            behavior: Box::new(behavior),
        }
    }
}

impl ViewController<'_> {
    /// Called at the end of window construction (`Window::new`).
    ///
    /// This is the time for the application itself to start building up the
    /// view heirarchy.
    pub(crate) fn window_loaded(&self, view: View) {
        match self.state.get() {
            State::WillLoad => {
                self.state.set(State::DidLoad);
                self.behavior.view_did_load(view);
            },
            _ => {
                panic!("You cannot re-use a view controller between multiple windows");
            }
        }
    }

    /// Called by `WindowBehavior.set_needs_display`. I.e. when the window needs
    /// to render.
    ///
    /// If the window hasn't rendered before, then this is the time to notify
    /// the app that the view is going to appear.
    pub(crate) fn window_set_needs_display(&self, view: View) {
        match self.state.get() {
            State::WillLoad => {
                panic!("Window is about to display but is somehow not loaded");
            },
            State::DidLoad => {
                self.state.set(State::WillAppear);
                self.behavior.view_will_appear(view);
            },
            State::WillAppear => (),
            State::DidAppear => (),
            // State::WillDisappear => {
            //     panic!("TODO: what should happen here?")
            // },
            // State::DidDisappear => {
            //     panic!("TODO: what should happen here?")
            // }
        }
    }

    /// Called by `render::window_display` after the view has been drawn to
    /// screen.
    ///
    /// This will be called on every re-render; specifically we care on the
    /// first render whilst we're still on `WillAppear` state.
    pub(crate) fn window_displayed(&self, view: View) {
        match self.state.get() {
            State::WillLoad => {
                panic!("Window is displaying but is somehow not loaded");
            },
            State::DidLoad => {
                panic!("Window missed WillAppear somehow");
            },
            State::WillAppear => {
                self.state.set(State::DidAppear);
                self.behavior.view_did_appear(view);
            },
            // State::DidDisappear => {
            //     panic!("Disappeared but still rendering")
            // }
            _ => ()
        }
    }
}

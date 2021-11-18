use crate::graphics::{Context, Layer, Rectangle, Point, Size};
use crate::ui::view::{View, WeakView};
use crate::ui::Application;
use crate::ui::render;
use crate::ui::Color;
use crate::ui::view::{Behavior, DefaultBehavior};
use crate::ui::Timer;
use crate::ui::RunLoop;
use crate::ui::run_loop::Mode;
use std::rc::Rc;

pub struct WindowBehavior {
    view: WeakView,
    super_behavior: Box<dyn Behavior>,
    pub(crate) graphics_context: Rc<Context>
}

pub struct Window {}
impl Window {
    pub fn new(title: &str, frame: Rectangle) -> View {
        let default_behavior = DefaultBehavior {
            view: WeakView::none()
        };

        let context_frame = frame.clone();

        let graphics_context = Context::new(
            title,
            context_frame.position,
            context_frame.size
        );

        let window = WindowBehavior {
            view: WeakView::none(),
            super_behavior: Box::new(default_behavior),
            graphics_context: Rc::new(graphics_context)
        };

        let view = View::new_with_behavior(Box::new(window), frame);
        view.set_hidden(true);
        view.set_background_color(Color::white());

        let mut application = Application::borrow_mut();
        application.add_window(view.clone());

        view
    }
}

impl Behavior for WindowBehavior {
    fn super_behavior(&self) -> Option<&Box<dyn Behavior>> {
        Some(&self.super_behavior)
    }

    fn mut_super_behavior(&mut self) -> Option<&mut dyn Behavior> {
        Some(self.super_behavior.as_mut())
    }

    fn set_view(&mut self, view: WeakView) {
        self.view = view;
    }

    fn get_view(&self) -> &WeakView {
        &self.view
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_window(&self) -> bool {
        true
    }

    /// For the `WindowBehavior` specifically, this will actually add a timer to
    /// the main loop to request a render.
    fn set_needs_display(&self) {
        self.super_behavior().unwrap().set_needs_display();

        let window_view = self.get_view().upgrade().unwrap();

        let run_loop = RunLoop::borrow();
        let dirty_timer = Timer::new_once(move || render::window_display(window_view.clone()));
        run_loop.add_timer(dirty_timer, Mode::Default);
    }
}

impl PartialEq for WindowBehavior {
    fn eq(&self, rhs: &WindowBehavior) -> bool {
        self.view.id == rhs.view.id
    }
}

impl std::fmt::Debug for WindowBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.view.id.to_string();

        f.debug_tuple("")
         .field(&id)
         .finish()
    }
}

fn make_key_and_visible(view: &View) {
    let mut application = Application::borrow_mut();
    application.set_key_window(view);
    view.set_hidden(false);
}

use crate::graphics::{Context, Rectangle};
use crate::ui::{View, WeakView, ViewController};
use crate::ui::view::{Behavior, DefaultBehavior};
use crate::ui::application::Application;
use crate::ui::render;
use crate::ui::Color;
use crate::ui::timer::Timer;
use crate::ui::run_loop::RunLoop;
use std::option::Option;
use std::cell::RefCell;

pub struct WindowBehavior {
    view: WeakView,
    super_behavior: Box<dyn Behavior>,
    context: Context,
    pub(crate) view_controller: ViewController<'static>,

    /// The window's first responder. Default to the window itself. Overriden
    /// by a view calling `become_first_responder`.
    first_responder: RefCell<WeakView>,
}

pub struct Window {
    pub view: View
}

impl std::ops::Deref for Window {
    type Target = View;
    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl Window {
    pub fn new(title: &str, frame: Rectangle<i32, u32>, view_controller: ViewController<'static>) -> Window {
        let default_behavior = DefaultBehavior {
            view: WeakView::none()
        };

        let context_frame = frame.clone();

        let context = Context::new(
            title,
            context_frame.origin,
            context_frame.size
        );

        let window_behavior = WindowBehavior {
            view: WeakView::none(),
            super_behavior: Box::new(default_behavior),
            context: context,
            view_controller: view_controller,
            first_responder: RefCell::new(WeakView::none())
        };

        let view = View::new_with_behavior(Box::new(window_behavior), frame, "window");

        let window = Window { view: view.clone() };

        {
            let mut application = Application::borrow_mut();
            application.add_window(window.clone());
        }

        {
            let behavior = window.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<WindowBehavior>().unwrap();
            behavior.first_responder.replace(window.view.downgrade());
            let view_controller = &behavior.view_controller;
            view_controller.window_loaded(view);
        }

        window.view.set_hidden(true);
        window.view.set_background_color(Color::white());

        window
    }

    pub fn from_view(view: View) -> Window {
        // Downcast the behavior to essentially verify the view is a window.
        let _ = view.behavior.borrow().as_any().downcast_ref::<WindowBehavior>().unwrap();

        Window { view }
    }

    pub fn make_key_and_visible(&self) {
        let mut application = Application::borrow_mut();
        application.set_key_window(&self);
        self.set_hidden(false);
    }

    pub fn context(&self) -> Context {
        let behavior = self.view.behavior.borrow();
        let behavior = behavior.as_any().downcast_ref::<WindowBehavior>().unwrap();
        behavior.context.clone()
    }

    /// Returns the window's first responder.
    ///
    /// If there is no first responder, the window itself is returned.
    pub(crate) fn first_responder(&self) -> View {
        let behavior = self.view.behavior.borrow();
        let behavior = behavior.as_any().downcast_ref::<WindowBehavior>().unwrap();
        let first_responder = behavior.first_responder.borrow();

        if let Some(first_responder) = first_responder.upgrade() {
            first_responder
        } else {
            self.view.clone()
        }
    }

    pub(crate) fn replace_first_responder(&self, view: View) -> bool {
        // If there is a first responder, ask whether it wants to resign. If it
        // doesn't, then we can't replace it.
        if !self.first_responder().can_resign_first_responder() {
            return false;
        }

        let behavior = self.view.behavior.borrow();
        let behavior = behavior.as_any().downcast_ref::<WindowBehavior>().unwrap();
        behavior.first_responder.replace(view.downgrade());
        return true;
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

        let window_view = self.view.upgrade().unwrap();
        {
            let behavior = window_view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<WindowBehavior>().unwrap();
            let vc = &behavior.view_controller;
            vc.window_set_needs_display(window_view.clone());
        }

        let run_loop = RunLoop::borrow();
        // TODO: would this benefit from Window rather than View?
        let dirty_timer = Timer::new_once(move || render::window_display(window_view.clone()));
        run_loop.add_timer(dirty_timer);
    }
}

impl PartialEq for WindowBehavior {
    fn eq(&self, rhs: &WindowBehavior) -> bool {
        self.view.id() == rhs.view.id()
    }
}

impl std::fmt::Debug for WindowBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.view.id();

        f.debug_tuple("")
         .field(&id)
         .finish()
    }
}

impl PartialEq for Window {
    fn eq(&self, rhs: &Window) -> bool {
        self.view.id() == rhs.view.id()
    }
}

impl std::fmt::Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.id();

        f.debug_tuple("")
         .field(&id)
         .finish()
    }
}

impl Clone for Window {
    fn clone(&self) -> Self {
      Window {
        view: self.view.clone()
      }
    }
}

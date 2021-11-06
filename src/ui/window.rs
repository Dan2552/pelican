use crate::graphics::{Context, Rectangle};
use crate::ui::view::{View, WeakView};
use crate::ui::Color;
use crate::ui::view::{Behavior, ViewBehavior};
use crate::ui::Timer;
use crate::ui::RunLoop;
use crate::ui::run_loop::Mode;

// TODO:
// https://users.rust-lang.org/t/convert-generic-trait-t-back-to-struct/11581
// maybe the solution is View<ViewBehavior> / View<WindowBehavior> ?

pub struct WindowBehavior {
    view: WeakView<WindowBehavior>,
    super_behavior: Box<dyn Behavior>,

    graphics_context: Context
}

pub struct Window {}
impl Window {
    pub fn new(title: &str, frame: Rectangle) -> View<WindowBehavior> {
        let default_behavior = ViewBehavior {
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
            graphics_context
        };

        let view = View::new_with_behavior(Box::new(window), frame);
        view.set_hidden(true);
        view.set_background_color(Color::white());

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

    fn set_view(&mut self, view: WeakView<WindowBehavior>) {
        self.view = view;
    }

    fn get_view(&self) -> &WeakView<WindowBehavior> {
        &self.view
    }

    /// For the `WindowBehavior` specifically, this will actually add a timer to
    /// the main loop to request a render.
    fn set_needs_display(&self) {
        let view = &self.view.upgrade().unwrap();
        let inner_self = view.inner_self.borrow();
        let layer = inner_self.layer.unwrap();
        if layer.get_needs_display() {
            return;
        }

        layer.set_needs_display();

        let dirty_timer = Timer::new_once(window_display.clone());
        RunLoop::main().add_timer(dirty_timer, Mode::Default);
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

fn window_display() {

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;
    use crate::graphics::Point;
    use crate::graphics::Size;

    #[test]
    fn initialize() {
        let frame = Rectangle {
            position: Point { x: 10, y: 10 },
            size: Size { width: 50, height: 50 }
        };

        let window = Window::new("test", frame);

        {
            let window_behavior = window.behavior.borrow();
            let view = window_behavior.get_view();
            assert_eq!(window, view.upgrade().unwrap());
        }
    }

    #[test]
    /// Tests add_subview and superview
    fn parent_child_relationship() {
        let frame = Rectangle {
            position: Point { x: 10, y: 10 },
            size: Size { width: 50, height: 50 }
        };

        let mut view_parent = Window::new("test", frame.clone());
        let view_child = View::new(frame.clone());

        view_parent.add_subview(view_child.clone());

        let view_child1 = view_child.clone();
        let child_inner_self = &view_child1.inner_self.borrow();
        let childs_parent = &child_inner_self.superview;

        assert_eq!(view_parent, childs_parent.upgrade().unwrap());

        let inner_self = view_parent.inner_self.borrow();
        let contains_child = inner_self.subviews.contains(&view_child);
        assert_eq!(contains_child, true);
    }
}

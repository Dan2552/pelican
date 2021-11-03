use crate::graphics::Context;
use crate::graphics::Rectangle;
use crate::ui::view::{View, WeakView};
// use crate::ui::view::Child;
use crate::ui::Color;
use crate::ui::view::Behavior;
use crate::ui::view::ViewBehavior;

struct WindowBehavior {
    view: WeakView,
    super_behavior: Box<dyn Behavior>,

    graphics_context: Context
}

pub struct Window {}
impl Window {
    pub fn new(frame: Rectangle) -> View {
        let default_behavior = ViewBehavior {
            view: WeakView::none()
        };

        let context_frame = frame.clone();

        let graphics_context = Context::new(
            "hello world",
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

    fn set_view(&mut self, view: WeakView) {
        self.view = view;
    }

    fn get_view(&self) -> &WeakView {
        &self.view
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

        let window = Window::new(frame);

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

        let mut view_parent = Window::new(frame.clone());
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

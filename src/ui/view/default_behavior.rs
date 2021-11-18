use crate::ui::view::{View, WeakView, Behavior};
use crate::ui::Color;

pub struct DefaultBehavior {
    pub(crate) view: WeakView
}
impl Behavior for DefaultBehavior {
    fn set_view(&mut self, view: WeakView) {
        self.view = view;
    }

    fn get_view(&self) -> &WeakView {
        &self.view
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// Adds a child `View` to this `View`.
    ///
    /// Also sets the parent (`superview`) of the child view to this `View`.
    fn add_subview(&self, child: View) {
        let view = self.get_view().upgrade().unwrap().clone();

        let weak_self = view.downgrade();
        let mut inner_self = view.inner_self.borrow_mut();

        {
            let mut child_inner = child.inner_self.borrow_mut();

            // Set the child superview
            child_inner.superview = weak_self;
        }

        inner_self.subviews.push(child);
    }

    /// Request for this view to be redrawn soon.
    ///
    /// See `#draw`, which includes the instructions on what would actually be
    /// drawn to screen.
    fn set_needs_display(&self) {
        let view = self.view.upgrade().unwrap().clone();

        let mut inner_self = view.inner_self.borrow_mut();

        // The layer may not yet exist for this view if it's not drawn to the
        // context at least once. But this is ok, because when a layer is set
        // by `render::window_display()` it will be be implied needs display as
        // default.
        if let Some(layer) = &mut inner_self.layer {
            if layer.get_needs_display() {
                return;
            }

            layer.set_needs_display();

            if let Some(superview) = &inner_self.superview.upgrade() {
                superview.set_needs_display();
            }
        }
    }

    /// Defines what actually gets drawn to screen to represent this view.
    ///
    /// For example, the default `View` implementation simply draws the
    /// background color as a box of the size of the frame.
    fn draw(&self) {
        let view = self.view.upgrade().unwrap().clone();

        let mut inner_self = view.inner_self.borrow_mut();

        let color = inner_self.background_color.to_graphics_color();

        if let Some(layer) = &mut inner_self.layer {
            layer.clear_with_color(color);
        }
    }

    /// Change the background color for this view.
    fn set_background_color(&self, color: Color) {
        {
            let view = self.view.upgrade().unwrap().clone();

            let mut inner_self = view.inner_self.borrow_mut();

            inner_self.background_color = color;
        }
        self.set_needs_display();
    }

    fn set_hidden(&self, value: bool) {
        let view = self.view.upgrade().unwrap().clone();

        let mut inner_self = view.inner_self.borrow_mut();
        inner_self.hidden = value;
    }
}
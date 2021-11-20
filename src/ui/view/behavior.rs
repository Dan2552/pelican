use crate::ui::view::{View, WeakView, ViewInner};
use crate::ui::Color;

pub trait Behavior {
    fn super_behavior(&self) -> Option<&Box<dyn Behavior>> {
        None
    }

    fn mut_super_behavior(&mut self) -> Option<&mut dyn Behavior> {
        None
    }

    fn set_super_behavior_view(&mut self, view: View) {
        if let Some(super_behavior) = self.mut_super_behavior() {
            super_behavior.set_view(view.downgrade());
            super_behavior.set_super_behavior_view(view);
        }
    }

    fn is_window(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any;

    fn set_view(&mut self, view: WeakView);
    fn get_view(&self) -> &WeakView;

    fn add_subview(&self, child: View) {
        if let Some(super_behavior) = self.super_behavior() {
            super_behavior.add_subview(child);
        } else {
            panic!("add_subview behavior not implemented. Have you implemented `super_behavior()`?");
        }
    }

    fn set_needs_display(&self) {
        if let Some(super_behavior) = self.super_behavior() {
            super_behavior.set_needs_display();
        } else {
            panic!("set_needs_display behavior not implemented. Have you implemented `super_behavior()`?")
        }
    }

    fn draw(&self) {
        if let Some(super_behavior) = self.super_behavior() {
            super_behavior.draw()
        } else {
            panic!("draw behavior not implemented. Have you implemented `super_behavior()`?")
        }
    }

    fn get_background_color(&self) -> &Color {
        if let Some(super_behavior) = self.super_behavior() {
            super_behavior.get_background_color()
        } else {
            panic!("get_background_color behavior not implemented. Have you implemented `super_behavior()`?")
        }
    }

    fn set_background_color(&self, color: Color) {
        if let Some(super_behavior) = self.super_behavior() {
            super_behavior.set_background_color(color);
        } else {
            panic!("set_background_color behavior not implemented. Have you implemented `super_behavior()`?")
        }
    }
    fn set_hidden(&self, value: bool) {
        if let Some(super_behavior) = self.super_behavior() {
            super_behavior.set_hidden(value);
        } else {
            panic!("set_hidden behavior not implemented. Have you implemented `super_behavior()`?")
        }
    }
}

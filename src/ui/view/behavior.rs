use crate::ui::view::{View, WeakView};

pub trait Behavior {
    fn super_behavior(&self) -> Option<&Box<dyn Behavior>> {
        panic!("super_behavior not implemented for {}", std::any::type_name::<Self>())
    }

    fn mut_super_behavior(&mut self) -> Option<&mut dyn Behavior> {
        panic!("mut_super_behavior not implemented for {}", std::any::type_name::<Self>())
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
}

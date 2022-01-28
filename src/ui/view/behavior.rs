use crate::ui::{View, WeakView, Touch};

pub trait Behavior {
    fn name(&self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    fn as_any(&self) -> &dyn std::any::Any;
    fn set_view(&mut self, view: WeakView);
    fn get_view(&self) -> &WeakView;
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

    fn touches_began(&self, _touches: &Vec<Touch>) {}
    fn touches_ended(&self, _touches: &Vec<Touch>) {}
    fn touches_moved(&self, _touches: &Vec<Touch>) {}

    /// Return `true` if the view can resign the first responder.
    ///
    /// Returns `true` by default.
    fn can_resign_first_responder(&self) -> bool {
        true
    }

    // TODO: default should propagate to next_responder
    // fn presses_began(&self, _presses: &Vec<Press>) {}

    // TODO: default should propagate to next_responder
    // fn presses_ended(&self, _presses: &Vec<Press>) {}

    /// Override this behavior if the view should accept text typing input. E.g.
    /// if the view is a text field. `TextField` utilizes this function.
    ///
    /// This differs from "presses", as it allows the OS to do some additional
    /// processing on the text input e.g. for different keyboard layouts and
    /// internationalization.
    ///
    /// For picking up individual key presses instead, use `presses_began`, etc.
    ///
    /// In order for a view to recieve this call, it must be a responder. In
    /// most cases, realistically a first responder. This can be requested by
    /// calling `become_first_responder` on the view.
    fn text_input_did_receive(&self, text: &str) {
        if let Some(next) = self.next_responder().upgrade() {
            next.text_input_did_receive(text);
        }
    }

    fn next_responder(&self) -> WeakView {
        if let Some(super_behavior) = self.super_behavior() {
            super_behavior.next_responder()
        } else {
            panic!("next_responder behavior not implemented. Have you implemented `super_behavior()`?")
        }
    }
}

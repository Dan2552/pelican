use crate::ui::Touch;
use crate::ui::WeakView;
use crate::ui::event::TouchEvent;

pub trait Recognizer {
    fn touches_began(&self, touches: &Vec<Touch>, event: &TouchEvent);
    fn touches_ended(&self, touches: &Vec<Touch>, event: &TouchEvent);
    fn touches_moved(&self, touches: &Vec<Touch>, event: &TouchEvent);

    /// If `true`, the recognizer can cancel touches sent to the view if it
    /// recognizes the touch is its gesture.
    ///
    /// To clarify, this doesn't mean the view will not see the touch at all.
    /// On the contrary, it will receive `touches_began`, alongside the
    /// recognizer receiving it too. Then, if the touch is recognized as a
    /// gesture, the view will receive `touches_cancelled`.
    ///
    /// Default to `true`.
    fn cancels_touches_in_view(&self) -> bool {
        true
    }

    /// This should only be called by `View.add_gesture_recognizer`.
    fn set_view(&self, view: WeakView);
}

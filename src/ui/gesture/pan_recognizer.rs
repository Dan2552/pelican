use crate::ui::{View, WeakView};
use crate::graphics::Point;
use std::cell::RefCell;
use crate::ui::gesture::recognizer::Recognizer;
use crate::ui::Touch;
use crate::ui::RunLoop;
use crate::ui::Timer;
use std::time::Duration;
use std::rc::Rc;
use crate::ui::event::TouchEvent;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum PanState {
    Possible,
    Began,
    Changed,
    Ended,
    Failed
}

pub struct PanRecognizer {
    inner: Rc<RefCell<PanRecognizerInner>>
}

struct PanRecognizerInner {
    view: WeakView,
    state: PanState,
    action: Box<dyn Fn(&PanRecognizer) -> ()>,
    translation: Point<i32>,
    start_position: Point<i32>
}

impl PanRecognizer {
    pub fn new(action: impl Fn(&PanRecognizer) -> () + 'static) -> PanRecognizer {
        PanRecognizer {
            inner: Rc::new(RefCell::new(PanRecognizerInner {
                view: WeakView::none(),
                state: PanState::Possible,
                action: Box::new(action),
                translation: Point::new(0, 0),
                start_position: Point::new(0, 0)
            }))
        }
    }

    fn translation_in(&self, view: &View) -> Point<i32> {
        let inner = self.inner.borrow();
        let translation = &inner.translation;

        if let Some(self_view) = inner.view.upgrade() {
            self_view.convert_point_to(&translation.clone(), view)
        } else {
            translation.clone()
        }
    }

    fn set_translation(&self, translation: Point<i32>, view: &View) {
        let mut inner = self.inner.borrow_mut();
        if let Some(self_view) = inner.view.upgrade() {
            inner.translation = view.convert_point_to(&translation, &self_view);
        } else {
            inner.translation = translation;
        }
    }

    fn state(&self) -> PanState {
        self.inner.borrow().state.clone()
    }

    fn view(&self) -> WeakView {
        self.inner.borrow().view.clone()
    }
}

impl Recognizer for PanRecognizer {
    fn touches_began(&self, touches: &Vec<Touch>, _event: &TouchEvent) {
        let mut inner = self.inner.borrow_mut();

        if inner.state == PanState::Failed || inner.state == PanState::Ended {
            inner.state = PanState::Possible;
        }

        if inner.state != PanState::Possible {
            return;
        }

        inner.start_position = touches.first().unwrap().position().clone();

        let timer_self = self.clone();
        let timer = Timer::new_once_delayed(Duration::from_millis(150), move || {
            let mut inner = timer_self.inner.borrow_mut();
            if inner.state == PanState::Possible {
                inner.state = PanState::Failed;
            }
        });
        RunLoop::borrow().add_timer(timer);
    }

    fn touches_ended(&self, _touches: &Vec<Touch>, _event: &TouchEvent) {
        let mut inner = self.inner.borrow_mut();
        inner.state = PanState::Ended;
    }

    fn touches_moved(&self, touches: &Vec<Touch>, _event: &TouchEvent) {
        let touch_position = touches.first().unwrap().position();
        let mut inner = self.inner.borrow_mut();

        inner.translation = Point::new(
            touch_position.x - inner.start_position.x,
            touch_position.y - inner.start_position.y
        );

        match inner.state {
            PanState::Possible => {
                if inner.translation.x.abs() > 10 || inner.translation.y.abs() > 10 {
                    inner.state = PanState::Began;
                } else {
                    return;
                }
            },
            PanState::Began => { inner.state = PanState::Changed },
            _ => {
                return;
            }
        }

        (inner.action)(self);
    }

    fn set_view(&self, view: WeakView) {
        let mut inner = self.inner.borrow_mut();
        inner.view = view;
    }
}

impl Clone for PanRecognizer {
    fn clone(&self) -> PanRecognizer {
        PanRecognizer {
            inner: self.inner.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;

    #[test]
    fn test_default_state() {
        let recognizer = PanRecognizer::new(|_pan_recognizer| {});
        assert_eq!(recognizer.state(), PanState::Possible);
    }

    #[test]
    fn test_default_view() {
        let recognizer = PanRecognizer::new(|_pan_recognizer| {});
        assert!(recognizer.view().is_none());
    }

    #[test]
    fn test_default_translation() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let view = View::new(frame);
        let recognizer = PanRecognizer::new(|_pan_recognizer| {});
        assert_eq!(recognizer.translation_in(&view), Point::new(0, 0));
    }

    #[test]
    fn test_translation() {
        {
            let recognizer = PanRecognizer::new(|_pan_recognizer| {});
            let frame = Rectangle::new(0, 0, 100, 100);
            let view = View::new(frame);
            recognizer.set_translation(Point::new(1, 2), &view);
            assert_eq!(recognizer.translation_in(&view), Point::new(1, 2));
        }

        {
            let recognizer = PanRecognizer::new(|_pan_recognizer| {});
            let view = View::new(Rectangle::new(0, 0, 100, 100));
            view.add_gesture_recognizer(Box::new(recognizer.clone()));
            recognizer.set_translation(Point::new(1, 2), &view);
            assert_eq!(recognizer.translation_in(&view), Point::new(1, 2));
        }

        {
            let parent_view = View::new(Rectangle::new(0, 0, 100, 100));
            let child_view = View::new(Rectangle::new(10, 10, 10, 10));
            parent_view.add_subview(child_view.clone());

            let recognizer = PanRecognizer::new(|_pan_recognizer| {});
            parent_view.add_gesture_recognizer(Box::new(recognizer.clone()));
            recognizer.set_translation(Point::new(1, 2), &child_view);
            assert_eq!(recognizer.translation_in(&child_view), Point::new(1, 2));
            assert_eq!(recognizer.translation_in(&parent_view), Point::new(11, 12));
        }
    }

    #[test]
    fn test_touches_began() {
        let recognizer = PanRecognizer::new(|_pan_recognizer| {});
        let frame = Rectangle::new(0, 0, 100, 100);
        let view = View::new(frame);
        view.add_gesture_recognizer(Box::new(recognizer.clone()));

        let event = TouchEvent::new();
        let touches = vec![Touch::new(0, Point::new(10, 10))];
        recognizer.touches_began(&touches, &event);

        assert_eq!(recognizer.state(), PanState::Possible);
    }

    #[test]
    fn test_touches_began_when_previously_failed() {
        //TODO: assert_eq!(true, false)
    }
}

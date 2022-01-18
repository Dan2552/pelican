use crate::ui::{View, WeakView};
use crate::graphics::Point;
use std::cell::RefCell;
use crate::ui::gesture::recognizer::Recognizer;
use crate::ui::Touch;
use std::rc::Rc;
use crate::ui::event::{TouchEvent, ScrollEvent};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum PanState {
    Possible,
    Began,
    Changed,
    Ended,
    Cancelled,
    Failed
}

pub struct PanRecognizer {
    inner: Rc<RefCell<PanRecognizerInner>>
}

struct PanRecognizerInner {
    view: WeakView,
    state: PanState,
    action: Rc<Box<dyn Fn(&PanRecognizer) -> ()>>,
    translation: Point<i32>,
    initial_position: Point<i32>,
    last_position: Point<i32>
}

impl PanRecognizer {
    pub fn new(action: impl Fn(&PanRecognizer) -> () + 'static) -> PanRecognizer {
        PanRecognizer {
            inner: Rc::new(RefCell::new(PanRecognizerInner {
                view: WeakView::none(),
                state: PanState::Possible,
                action: Rc::new(Box::new(action)),
                translation: Point::new(0, 0),
                initial_position: Point::new(0, 0),
                last_position: Point::new(0, 0)
            }))
        }
    }

    pub fn translation_in(&self, _view: &View) -> Point<i32> {
        let inner = self.inner.borrow();
        let translation = &inner.translation;
        translation.clone()
    }

    pub fn set_translation(&self, translation: Point<i32>, _view: &View) {
        let mut inner = self.inner.borrow_mut();
        inner.translation = translation;
        inner.initial_position = inner.last_position.clone();
    }

    pub fn state(&self) -> PanState {
        self.inner.borrow().state.clone()
    }

    pub fn view(&self) -> WeakView {
        self.inner.borrow().view.clone()
    }
}

impl Recognizer for PanRecognizer {
    fn touches_began(&self, touches: &Vec<Touch>, _event: &TouchEvent) {
        let mut inner = self.inner.borrow_mut();

        inner.state = PanState::Possible;

        inner.last_position = touches.first().unwrap().position().clone();
        inner.initial_position = inner.last_position.clone();
        inner.translation = Point::new(0, 0);
    }

    fn touches_ended(&self, _touches: &Vec<Touch>, _event: &TouchEvent) {
        let mut inner = self.inner.borrow_mut();
        inner.state = PanState::Ended;
    }

    fn touches_moved(&self, touches: &Vec<Touch>, _event: &TouchEvent) {
        let action: Rc<Box<dyn Fn(&PanRecognizer) -> ()>>;
        {
            let touch_position = touches.first().unwrap().position();
            let mut inner = self.inner.borrow_mut();

            inner.translation = Point::new(
                touch_position.x - inner.initial_position.x,
                touch_position.y - inner.initial_position.y
            );

            inner.last_position = touch_position;

            match inner.state {
                PanState::Possible => {
                    if inner.translation.x.abs() > 10 || inner.translation.y.abs() > 10 {
                        inner.state = PanState::Began;
                    } else {
                        return;
                    }
                },
                PanState::Began => { inner.state = PanState::Changed },
                PanState::Changed => {},
                _ => {
                    return;
                }
            }

            action = inner.action.clone();
        }

        action(self);
    }

    fn scroll_did_translate(&self, translation: &Point<i32>, event: &ScrollEvent) {
        println!("scroll_did_translate: {:?}", translation);

        let action: Rc<Box<dyn Fn(&PanRecognizer) -> ()>>;
        {
            let mut inner = self.inner.borrow_mut();
            inner.translation = Point::new(
                -translation.x,
                translation.y
            );
            // inner.initial_position = inner.last_position.clone();
            // inner.last_position = event.position().clone();
            action = inner.action.clone();
        }
        action(self);
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
            assert_eq!(recognizer.translation_in(&parent_view), Point::new(1, 2));
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

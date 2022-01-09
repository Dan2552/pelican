use crate::graphics::Point;
use crate::ui::{View, Window};
use std::time::Instant;
use crate::ui::gesture::recognizer::Recognizer;
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};

struct TouchInner {
    id: usize,
    timestamp: Instant,
    position: Point<i32>,
    phase: TouchPhase,
    view: Option<View>,
    window: Option<Window>,
    gesture_recognizers: Vec<Weak<Box<dyn Recognizer>>>
}

pub struct Touch {
    inner: Rc<RefCell<TouchInner>>
}

/// A touch phase describes the lifecycle of a touch event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TouchPhase {
    Began,
    Moved,
    Ended,
    Cancelled
}

impl Touch {
    pub fn new(id: usize, position: Point<i32>) -> Touch {
        Touch {
            inner: Rc::new(RefCell::new(TouchInner {
                id,
                timestamp: Instant::now(),
                position,
                phase: TouchPhase::Began,
                view: None,
                window: None,
                gesture_recognizers: Vec::new()
            }))
        }
    }

    pub fn position(&self) -> Point<i32> {
        self.inner.borrow().position.clone()
    }

    pub(crate) fn set_position(&self, position: Point<i32>) {
        self.inner.borrow_mut().position = position;
    }

    pub(crate) fn set_view(&self, view: View) {
        self.inner.borrow_mut().view = Some(view);
    }

    pub fn view(&self) -> Option<View> {
        let inner = self.inner.borrow();
        if let Some(view) = &inner.view {
            Some(view.clone())
        } else {
            None
        }
    }

    pub(crate) fn set_window(&self, window: Window) {
        self.inner.borrow_mut().window = Some(window);
    }

    pub(crate) fn gesture_recognizers(&self) -> Ref<'_, Vec<Weak<Box<dyn Recognizer>>>> {
        // &self.inner.borrow().gesture_recognizers
        Ref::map(self.inner.borrow(), |inner| &inner.gesture_recognizers)
    }

    pub(crate) fn set_gesture_recognizers(&self, recognizers: Vec<Weak<Box<dyn Recognizer>>>) {
        self.inner.borrow_mut().gesture_recognizers = recognizers;
    }

    pub fn window(&self) -> Option<Window> {
        self.inner.borrow().window.clone()
    }

    pub fn phase(&self) -> TouchPhase {
        self.inner.borrow().phase
    }

    pub(crate) fn set_phase(&mut self, phase: TouchPhase) {
        self.inner.borrow_mut().phase = phase;
    }

    pub(crate) fn set_timestamp(&mut self, timestamp: Instant) {
        self.inner.borrow_mut().timestamp = timestamp;
    }

    pub(crate) fn timestamp(&self) -> Instant {
        self.inner.borrow().timestamp
    }

    pub fn id(&self) -> usize {
        self.inner.borrow().id
    }

    pub(crate) fn update_timestamp(&mut self) {
        self.inner.borrow_mut().timestamp = Instant::now();
    }
}

impl PartialEq for Touch {
    fn eq(&self, other: &Touch) -> bool {
        self.id() == other.id()
    }
}

impl std::fmt::Debug for Touch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Touch")
         .field(&self.id())
         .finish()
    }
}

impl Clone for Touch {
    fn clone(&self) -> Touch {
        Touch {
            inner: self.inner.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;
    use crate::ui::gesture::pan_recognizer::PanRecognizer;
    use std::rc::Rc;

    #[test]
    fn test_get_position() {
        let touch = Touch::new(0, Point { x: 5, y: 5 });
        assert_eq!(touch.position(), Point { x: 5, y: 5 });
    }

    #[test]
    fn test_eq() {
        let touch1 = Touch::new(0, Point { x: 5, y: 5 });
        let touch2 = Touch::new(0, Point { x: 5, y: 5 });
        assert_eq!(touch1, touch2);
    }

    #[test]
    fn test_debug() {
        let touch = Touch::new(0, Point { x: 5, y: 5 });
        assert_eq!(format!("{:?}", touch), "Touch(0)");
    }

    #[test]
    fn test_get_id() {
        let touch = Touch::new(2, Point { x: 5, y: 5 });
        assert_eq!(touch.id(), 2);
    }

    #[test]
    fn test_get_view() {
        let view = View::new(Rectangle::new(0, 0, 100, 100));
        let mut touch = Touch::new(0, Point { x: 5, y: 5 });
        touch.set_view(view.clone());
        assert_eq!(touch.view(), Some(view));
    }

    #[test]
    fn test_update_timestamp() {
        let mut touch = Touch::new(0, Point { x: 5, y: 5 });
        let original_time = touch.timestamp();
        std::thread::sleep(std::time::Duration::from_millis(10));
        touch.update_timestamp();
        assert!(touch.timestamp() != original_time);
    }

    #[test]
    fn test_set_phase() {
        let mut touch = Touch::new(0, Point { x: 5, y: 5 });
        touch.set_phase(TouchPhase::Moved);
        assert_eq!(touch.phase(), TouchPhase::Moved);
    }

    #[test]
    fn test_phase() {
        let touch = Touch::new(0, Point { x: 5, y: 5 });
        assert_eq!(touch.phase(), TouchPhase::Began);
    }

    #[test]
    fn test_set_recognizers() {
        let mut touch = Touch::new(0, Point { x: 5, y: 5 });
        let recognizer: Box<dyn Recognizer> = Box::new(PanRecognizer::new(|_| {}));
        let recognizer = Rc::new(recognizer);
        let weak_recognizer = Rc::downgrade(&recognizer);
        assert_eq!(touch.gesture_recognizers().len(), 0);
        touch.set_gesture_recognizers(vec![weak_recognizer]);
        assert_eq!(touch.gesture_recognizers().len(), 1);
    }

    #[test]
    fn test_clone() {
        let touch = Touch::new(0, Point { x: 5, y: 5 });
        let clone = touch.clone();
        assert_eq!(touch, clone);
    }
}

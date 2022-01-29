use crate::ui::Touch;
use crate::macros::*;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use crate::ui::touch::TouchPhase;
use crate::graphics::Point;
use crate::ui::key::Key;
use crate::ui::press::Press;

struct TouchEventInner {
    touches: Vec<Touch>
}

pub struct TouchEvent {
    inner: Rc<RefCell<TouchEventInner>>
}

impl TouchEvent {
    pub(crate) fn new() -> TouchEvent {
        TouchEvent {
            inner: Rc::new(RefCell::new(TouchEventInner {
                touches: Vec::new()
            }))
        }
    }

    pub fn touches(&self) -> Ref<'_, Vec<Touch>> {
        Ref::map(self.inner.borrow(), |inner| &inner.touches)
    }
}

impl Clone for TouchEvent {
    fn clone(&self) -> Self {
        TouchEvent {
            inner: self.inner.clone()
        }
    }
}

struct ScrollEventInner {
    // While a scroll doesn't really have a "touch" per-se, we can use one to
    // keep track of the original cursor position of when the scroll started,
    // and most importantly which view is being scrolled (which would be
    // determined the same way a touch would, through `hit_test`).
    touch: Touch,
    translation: Point<i32>
}

pub struct ScrollEvent {
    inner: Rc<RefCell<ScrollEventInner>>
}

impl ScrollEvent {
    pub(crate) fn new() -> ScrollEvent {
        let touch = Touch::new(0, Point::new(0, 0));
        let translation = Point::new(0, 0);
        ScrollEvent {
            inner: Rc::new(RefCell::new(ScrollEventInner {
                touch,
                translation
            }))
        }
    }

    pub fn touch(&self) -> Touch {
        self.inner.borrow().touch.clone()
    }

    pub fn translation(&self) -> Point<i32> {
        self.inner.borrow().translation.clone()
    }
}

impl Clone for ScrollEvent {
    fn clone(&self) -> Self {
        ScrollEvent {
            inner: self.inner.clone()
        }
    }
}

pub struct PressEvent {
    inner: Rc<PressEventInner>
}

struct PressEventInner {
    press: Press
}

impl PressEvent {
    fn new(key: Key) -> PressEvent {
        let press = Press::new(key);

        PressEvent {
            inner: Rc::new(PressEventInner {
                press
            })
        }
    }

    pub fn press(&self) -> &Press {
        &self.inner.press
    }
}

impl Clone for PressEvent {
    fn clone(&self) -> Self {
        PressEvent {
            inner: self.inner.clone()
        }
    }
}

singleton!(EventArena, touch_event: None, scroll_event: None, press_events: Vec::new());

pub(crate) struct EventArena {
    touch_event: Option<TouchEvent>,
    scroll_event: Option<ScrollEvent>,
    press_events: Vec<PressEvent>
}

impl EventArena {
    pub(crate) fn touch_event(&mut self) -> TouchEvent {
        if self.touch_event.is_none() {
            self.touch_event = Some(TouchEvent::new());
        }

        self.touch_event.as_ref().unwrap().clone()
    }

    pub(crate) fn scroll_event(&mut self) -> ScrollEvent {
        if self.scroll_event.is_none() {
            self.scroll_event = Some(ScrollEvent::new());
        }

        self.scroll_event.as_ref().unwrap().clone()
    }

    pub(crate) fn press_began(&mut self, key: Key) -> PressEvent {
        let event = PressEvent::new(key);
        self.press_events.push(event.clone());
        event
    }

    pub(crate) fn touch_began(&mut self, touch: Touch) -> TouchEvent {
        let event = self.touch_event();
        if event.touches().contains(&touch) {
            panic!("Touch only just began but it already exists");
        }
        event.inner.borrow_mut().touches.push(touch);
        event
    }

    pub(crate) fn touch_moved(&mut self, touch_id: usize, position: Point<i32>) {
        let event = self.touch_event();

        for t in event.inner.borrow_mut().touches.iter_mut() {
            if t.id() == touch_id {
                if t.phase() == TouchPhase::Began || t.phase() == TouchPhase::Moved {
                    t.set_phase(TouchPhase::Moved);
                    t.set_position(position);
                }
                return;
            }
        }

        let scroll_event = self.scroll_event();
        let touch = scroll_event.touch();
        touch.set_position(position);
    }

    pub(crate) fn touch_ended(&mut self, touch_id: usize, position: Point<i32>) {
        let event = self.touch_event();

        for t in event.inner.borrow_mut().touches.iter_mut() {
            if t.id() == touch_id {
                t.set_phase(TouchPhase::Ended);
                t.set_position(position);
                return;
            }
        }

        panic!("Touch just ended but it doesn't exist");
    }

    pub(crate) fn scroll_did_translate(&mut self, translation: Point<i32>) {
        let event = self.scroll_event();
        event.inner.borrow_mut().translation = translation;
    }

    /// Clears out any touches that have ended.
    ///
    /// This is called by the start of the event loop.
    pub(crate) fn cleanup_ended_touches(&mut self) {
        let event = self.touch_event();
        event.inner.borrow_mut().touches.retain(|t| t.phase() != TouchPhase::Ended);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Point;

    #[test]
    fn test_touch_event_clone() {
        let touch_event = TouchEvent::new();
        let touch_event_clone = touch_event.clone();

        let touch = Touch::new(0, Point::new(0, 0));
        touch_event.inner.borrow_mut().touches.push(touch);

        assert_eq!(touch_event.touches().len(), 1);
        for (index, touch) in touch_event.touches().iter().enumerate() {
            assert_eq!(touch.id(), touch_event.touches()[index].id());
            assert_eq!(touch.id(), touch_event_clone.touches()[index].id());
        }
    }

    #[test]
    fn test_event_arena_touch_event() {
        let mut arena = EventArena { touch_event: None, scroll_event: None, press_events: Vec::new() };
        let touch_event = arena.touch_event();
        assert_eq!(touch_event.touches().len(), 0);
    }

    #[test]
    fn test_event_arena_touch_began() {
        let mut arena = EventArena { touch_event: None, scroll_event: None, press_events: Vec::new() };
        let touch = Touch::new(0, Point::new(0, 0));
        arena.touch_began(touch);
        let touch_event = arena.touch_event();
        assert_eq!(touch_event.touches().len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_event_arena_touch_began_twice() {
        let mut arena = EventArena { touch_event: None, scroll_event: None, press_events: Vec::new() };
        let touch = Touch::new(0, Point::new(0, 0));
        arena.touch_began(touch);
        let touch = Touch::new(0, Point::new(0, 0));
        arena.touch_began(touch);
    }

    #[test]
    fn test_event_arena_touch_moved() {
        let mut arena = EventArena { touch_event: None, scroll_event: None, press_events: Vec::new() };
        let touch = Touch::new(0, Point::new(0, 0));
        arena.touch_began(touch);
        arena.touch_moved(0, Point::new(10, 50));
        let touch_event = arena.touch_event();
        assert_eq!(touch_event.touches().len(), 1);
        assert_eq!(touch_event.touches()[0].position(), Point::new(10, 50));
    }

    #[test]
    fn test_event_arena_touch_ended() {
        let mut arena = EventArena { touch_event: None, scroll_event: None, press_events: Vec::new() };
        let touch = Touch::new(0, Point::new(0, 0));
        arena.touch_began(touch);
        arena.touch_ended(0, Point::new(10, 50));
        let touch_event = arena.touch_event();
        assert_eq!(touch_event.touches().len(), 1);
        assert_eq!(touch_event.touches()[0].position(), Point::new(10, 50));
        assert_eq!(touch_event.touches()[0].phase(), TouchPhase::Ended);
    }

    #[test]
    #[should_panic]
    fn test_event_arena_touch_ended_but_didnt_exist() {
        let mut arena = EventArena { touch_event: None, scroll_event: None, press_events: Vec::new() };
        arena.touch_ended(0, Point::new(10, 50));
    }

    #[test]
    fn test_event_arena_cleanup_ended_touches() {
        let mut arena = EventArena { touch_event: None, scroll_event: None, press_events: Vec::new() };
        let touch = Touch::new(0, Point::new(0, 0));
        arena.touch_began(touch);
        let touch = Touch::new(1, Point::new(0, 0));
        arena.touch_began(touch);
        assert_eq!(arena.touch_event().touches().len(), 2);
        arena.cleanup_ended_touches();
        assert_eq!(arena.touch_event().touches().len(), 2);
        arena.touch_ended(0, Point::new(10, 50));
        arena.cleanup_ended_touches();
        assert_eq!(arena.touch_event().touches().len(), 1);
    }
}

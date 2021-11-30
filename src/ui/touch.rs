use crate::graphics::Point;
use crate::ui::{View, Window};
use std::time::Instant;

pub struct Touch {
    id: i32,
    timestamp: Instant,
    position: Point<i32>,
    phase: TouchPhase,
    view: Option<View>,
    window: Option<Window>,
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
    pub fn new(id: i32, position: Point<i32>, phase: TouchPhase) -> Touch {
        Touch {
            id,
            timestamp: Instant::now(),
            position,
            phase,
            view: None,
            window: None
        }
    }

    pub fn get_position(&self) -> &Point<i32> {
        &self.position
    }

    pub(crate) fn set_position(&mut self, position: Point<i32>) {
        self.position = position;
    }

    pub(crate) fn set_view(&mut self, view: View) {
        self.view = Some(view);
    }

    pub fn get_view(&self) -> Option<&View> {
        self.view.as_ref()
    }

    pub(crate) fn set_window(&mut self, window: Window) {
        self.window = Some(window);
    }

    pub fn get_window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub(crate) fn update_timestamp(&mut self) {
        self.timestamp = Instant::now();
    }
}

impl PartialEq for Touch {
    fn eq(&self, other: &Touch) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Debug for Touch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Touch")
         .field(&self.id)
         .finish()
    }
}

impl Clone for Touch {
    fn clone(&self) -> Touch {
        Touch {
            id: self.id,
            timestamp: self.timestamp.clone(),
            position: self.position.clone(),
            phase: self.phase.clone(),
            view: self.view.clone(),
            window: self.window.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;

    #[test]
    fn test_get_position() {
        let touch = Touch::new(0, Point { x: 5, y: 5 }, TouchPhase::Began);
        assert_eq!(touch.get_position(), &Point { x: 5, y: 5 });
    }

    fn test_eq() {
        let touch1 = Touch::new(0, Point { x: 5, y: 5 }, TouchPhase::Began);
        let touch2 = Touch::new(0, Point { x: 5, y: 5 }, TouchPhase::Began);
        assert_eq!(touch1, touch2);
    }

    fn test_debug() {
        let touch = Touch::new(0, Point { x: 5, y: 5 }, TouchPhase::Began);
        assert_eq!(format!("{:?}", touch), "Touch(0)");
    }

    fn test_get_id() {
        let touch = Touch::new(2, Point { x: 5, y: 5 }, TouchPhase::Began);
        assert_eq!(touch.get_id(), 2);
    }

    fn test_get_view() {
        let view = View::new(Rectangle::new(0, 0, 100, 100));
        let mut touch = Touch::new(0, Point { x: 5, y: 5 }, TouchPhase::Began);
        touch.set_view(view.clone());
        assert_eq!(touch.get_view(), Some(&view));
    }

    fn test_update_timestamp() {
        let mut touch = Touch::new(0, Point { x: 5, y: 5 }, TouchPhase::Began);
        let original_time = touch.timestamp;
        touch.update_timestamp();
        assert!(touch.timestamp != original_time);
    }
}

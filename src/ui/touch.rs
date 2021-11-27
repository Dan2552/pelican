use crate::graphics::Point;

pub struct Touch {
    id: i32,
    point: Point<i32>,
    phase: TouchPhase,
}

/// A touch phase describes the lifecycle of a touch event.
pub enum TouchPhase {
    Began,
    Moved,
    Ended,
    Cancelled
}

impl Touch {
    pub fn new(id: i32, point: Point<i32>, phase: TouchPhase) -> Touch {
        Touch {
            id,
            point,
            phase
        }
    }

    pub fn get_position(&self) -> &Point<i32> {
        &self.point
    }
}

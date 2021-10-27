use crate::graphics::Point;
use crate::graphics::Size;

pub struct Rectangle {
  pub position: Point,
  pub size: Size
}

impl Clone for Rectangle {
  fn clone(&self) -> Self {
    Rectangle {
      position: self.position.clone(),
      size: self.size.clone()
    }
  }
}

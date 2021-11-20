pub struct Point<T> {
  pub x: T,
  pub y: T
}

impl<T> Clone for Point<T> where T: Copy {
  fn clone(&self) -> Self {
    Point {
      x: self.x,
      y: self.y
    }
  }
}

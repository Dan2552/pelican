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

impl<T> PartialEq for Point<T> where T: PartialEq {
  fn eq(&self, rhs: &Point<T>) -> bool {
      self.x == rhs.x && self.y == rhs.y
  }
}

impl<T> std::fmt::Debug for Point<T> where T: std::fmt::Debug {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_tuple("")
       .field(&self.x)
       .field(&self.y)
       .finish()
  }
}

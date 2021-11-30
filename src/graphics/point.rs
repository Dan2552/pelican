use crate::graphics::Number;

pub struct Point<T> where T: Number {
    pub x: T,
    pub y: T
}

impl<T> Point<T> where T: Number {
    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y
        }
    }
}

impl<T> Clone for Point<T> where T: Number {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y
        }
    }
}

impl<T> PartialEq for Point<T> where T: Number {
    fn eq(&self, rhs: &Point<T>) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl<T> std::fmt::Debug for Point<T> where T: Number {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Point")
        .field(&self.x)
        .field(&self.y)
        .finish()
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
      let p1 = Point { x: 1, y: 2 };
      let p2 = p1.clone();
      assert_eq!(p1, p2);
    }
  
    #[test]
    fn test_eq() {
      let p1 = Point { x: 1, y: 2 };
      let p2 = Point { x: 1, y: 2 };
      assert_eq!(p1, p2);
    }
  
    #[test]
    fn test_debug() {
      let p1 = Point { x: 1, y: 2 };
      assert_eq!(format!("{:?}", p1), "Point(1, 2)");
    }
}

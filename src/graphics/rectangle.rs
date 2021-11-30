use crate::graphics::Point;
use crate::graphics::Size;
use crate::graphics::Number;
use std::ops::Mul;

pub struct Rectangle<T, U> where T: Number, U: Number {
    pub position: Point<T>,
    pub size: Size<U>
}

impl<T, U> Rectangle<T, U> where T: Number, U: Number {
    pub fn new(x: T, y: T, width: U, height: U) -> Self {
        Rectangle {
            position: Point { x, y },
            size: Size { width, height }
        }
    }
}

impl Rectangle<i32, u32> {
    pub fn contains(&self, point: &Point<i32>) -> bool {
        point.x >= self.position.x && point.y >= self.position.y &&
            point.x <= self.position.x + self.size.width as i32 &&
            point.y <= self.position.y + self.size.height as i32
    }
}

impl<T, U> Clone for Rectangle<T, U> where T: Number, U: Number {
    fn clone(&self) -> Self {
        Rectangle {
            position: self.position.clone(),
            size: self.size.clone()
        }
    }
}

impl Mul<f32> for &Rectangle<i32, i32> {
    type Output = Rectangle<i32, i32>;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.position.x as f32 * rhs;
        let y = self.position.y as f32 * rhs;
        let width = self.size.width as f32 * rhs;
        let height = self.size.height as f32 * rhs;

        Rectangle {
        position: Point { x: x.round() as i32, y: y.round() as i32 },
        size: Size { width: width.round() as i32, height: height.round() as i32 }
        }
    }
}

impl Mul<f32> for &Rectangle<i32, u32> {
  type Output = Rectangle<i32, u32>;
  fn mul(self, rhs: f32) -> Self::Output {
        let x = self.position.x as f32 * rhs;
        let y = self.position.y as f32 * rhs;
        let width = self.size.width as f32 * rhs;
        let height = self.size.height as f32 * rhs;

        Rectangle {
            position: Point { x: x.round() as i32, y: y.round() as i32 },
            size: Size { width: width.round() as u32, height: height.round() as u32 }
        }
  }
}

impl Mul<f32> for &Rectangle<f32, f32> {
    type Output = Rectangle<f32, f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.position.x * rhs;
        let y = self.position.y * rhs;
        let width = self.size.width * rhs;
        let height = self.size.height * rhs;

        Rectangle {
            position: Point { x: x, y: y },
            size: Size { width: width, height: height }
        }
    }
}

impl<T, U> PartialEq for Rectangle<T, U> where T: Number, U: Number {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size
    }
}

impl<T, U> std::fmt::Debug for Rectangle<T, U> where T: Number, U: Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Rectangle")
         .field(&self.position.x)
         .field(&self.position.y)
         .field(&self.size.width)
         .field(&self.size.height)
         .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_contains() {
        let rect = Rectangle::new(0, 0, 100, 100);
        let point = Point { x: 50, y: 50 };

        assert!(rect.contains(&point));
        assert!(!rect.contains(&Point { x: -1, y: -1 }));
        assert!(!rect.contains(&Point { x: 101, y: 101 }));
        assert!(!rect.contains(&Point { x: -1, y: 101 }));
        assert!(!rect.contains(&Point { x: 101, y: -1 }));
    }

    fn test_multiply() {
        let rect: Rectangle<i32, u32> = Rectangle::new(0, 0, 100, 100);
        let multiplied = &rect * 2.0;

        assert_eq!(multiplied, Rectangle::new(0, 0, 200, 200));

        let rect: Rectangle<i32, u32> = Rectangle::new(0, 0, 100, 100);
        let multiplied = &rect * 2.0;

        assert_eq!(multiplied, Rectangle::new(0, 0, 200, 200));

        let rect: Rectangle<f32, f32> = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        let multiplied = &rect * 2.0;

        assert_eq!(multiplied, Rectangle::new(0.0, 0.0, 200.0, 200.0));
    }

    fn test_eq() {
        let rect1 = Rectangle::new(0, 0, 100, 100);
        let rect2 = Rectangle::new(0, 0, 100, 100);
        let rect3 = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect1, rect2);
        assert_eq!(rect1, rect3);

        let rect1 = Rectangle::new(0, 0, 100, 100);
        let rect2 = Rectangle::new(0, 1, 100, 100);

        assert_ne!(rect1, rect2);

        let rect1 = Rectangle::new(0, 0, 100, 100);
        let rect2 = Rectangle::new(0, 0, 1, 100);

        assert_ne!(rect1, rect2);
    }
}

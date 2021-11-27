use crate::graphics::Point;
use crate::graphics::Size;
use std::ops::Mul;

pub trait RectangleNumber: Copy {}
impl RectangleNumber for f32 {}
impl RectangleNumber for i32 {}
impl RectangleNumber for u32 {}

pub struct Rectangle<T, U> where T: RectangleNumber, U: RectangleNumber {
    pub position: Point<T>,
    pub size: Size<U>
}

impl<T, U> Rectangle<T, U> where T: RectangleNumber, U: RectangleNumber {
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

impl<T, U> Clone for Rectangle<T, U> where T: RectangleNumber, U: RectangleNumber {
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
}

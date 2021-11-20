use crate::graphics::Point;
use crate::graphics::Size;
use std::ops::Mul;

pub struct Rectangle<T, U> {
    pub position: Point<T>,
    pub size: Size<U>
}

impl<T, U> Rectangle<T, U> {
    pub fn new(x: T, y: T, width: U, height: U) -> Self {
        Rectangle {
            position: Point { x, y },
            size: Size { width, height }
        }
    }
}

impl<T, U> Clone for Rectangle<T, U> where T: Copy, U: Copy {
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

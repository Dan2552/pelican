use crate::graphics::Point;
use crate::graphics::Size;
use crate::graphics::Number;
use std::ops::Mul;

pub struct Rectangle<T, U> where T: Number, U: Number {
    pub origin: Point<T>,
    pub size: Size<U>
}

impl<T, U> Rectangle<T, U> where T: Number, U: Number {
    pub fn new(x: T, y: T, width: U, height: U) -> Self {
        Rectangle {
            origin: Point { x, y },
            size: Size { width, height }
        }
    }
}

impl Rectangle<i32, u32> {
    pub fn new_from_center(center: Point<i32>, size: Size<u32>) -> Self {
        let origin = Point {
            x: center.x - (size.width as f32 / 2.0) as i32,
            y: center.y - (size.height as f32 / 2.0) as i32
        };
        Rectangle {
            origin,
            size
        }
    }

    pub fn contains(&self, point: &Point<i32>) -> bool {
        point.x >= self.origin.x && point.y >= self.origin.y &&
            point.x <= self.origin.x + self.size.width as i32 &&
            point.y <= self.origin.y + self.size.height as i32
    }

    pub fn bottom(&self) -> i32 {
        self.origin.y + self.size.height as i32
    }

    pub fn right(&self) -> i32 {
        self.origin.x + self.size.width as i32
    }

    pub fn top(&self) -> i32 {
        self.origin.y
    }

    pub fn left(&self) -> i32 {
        self.origin.x
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn height(&self) -> u32 {
        self.size.height
    }

    pub fn center(&self) -> Point<i32> {
        Point {
            x: self.origin.x + self.size.width as i32 / 2,
            y: self.origin.y + self.size.height as i32 / 2
        }
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    pub fn origin(&self) -> &Point<i32> {
        &self.origin
    }
}

impl<T, U> Clone for Rectangle<T, U> where T: Number, U: Number {
    fn clone(&self) -> Self {
        Rectangle {
            origin: self.origin.clone(),
            size: self.size.clone()
        }
    }
}

impl Mul<f32> for &Rectangle<i32, i32> {
    type Output = Rectangle<i32, i32>;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.origin.x as f32 * rhs;
        let y = self.origin.y as f32 * rhs;
        let width = self.size.width as f32 * rhs;
        let height = self.size.height as f32 * rhs;

        Rectangle {
            origin: Point { x: x.round() as i32, y: y.round() as i32 },
            size: Size { width: width.round() as i32, height: height.round() as i32 }
        }
    }
}

impl Mul<f32> for &Rectangle<i32, u32> {
  type Output = Rectangle<i32, u32>;
  fn mul(self, rhs: f32) -> Self::Output {
        let x = self.origin.x as f32 * rhs;
        let y = self.origin.y as f32 * rhs;
        let width = self.size.width as f32 * rhs;
        let height = self.size.height as f32 * rhs;

        Rectangle {
            origin: Point { x: x.round() as i32, y: y.round() as i32 },
            size: Size { width: width.round() as u32, height: height.round() as u32 }
        }
  }
}

impl Mul<f32> for &Rectangle<f32, f32> {
    type Output = Rectangle<f32, f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.origin.x * rhs;
        let y = self.origin.y * rhs;
        let width = self.size.width * rhs;
        let height = self.size.height * rhs;

        Rectangle {
            origin: Point { x: x, y: y },
            size: Size { width: width, height: height }
        }
    }
}

impl<T, U> PartialEq for Rectangle<T, U> where T: Number, U: Number {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.size == other.size
    }
}

impl<T, U> std::fmt::Debug for Rectangle<T, U> where T: Number, U: Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Rectangle")
         .field(&self.origin.x)
         .field(&self.origin.y)
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

    #[test]
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

    #[test]
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

    #[test]
    fn test_bottom() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect.bottom(), 100);
    }

    #[test]
    fn test_top() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect.top(), 0);
    }

    #[test]
    fn test_left() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect.left(), 0);
    }

    #[test]
    fn test_right() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect.right(), 100);
    }

    #[test]
    fn test_width() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect.width(), 100);
    }

    #[test]
    fn test_height() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect.height(), 100);
    }

    #[test]
    fn test_center() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect.center(), Point { x: 50, y: 50 });
    }

    #[test]
    fn test_debug() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(format!("{:?}", rect), "Rectangle(0, 0, 100, 100)");
    }

    #[test]
    fn test_clone() {
        let rect = Rectangle::new(0, 0, 100, 100);
        let cloned = rect.clone();

        assert_eq!(rect, cloned);
    }

    #[test]
    fn new_from_center() {
        let center = Point { x: 50, y: 50 };
        let size = Size { width: 100, height: 100 };

        let rect = Rectangle::new_from_center(center, size);

        assert_eq!(rect, Rectangle::new(0, 0, 100, 100));
    }

    #[test]
    fn test_size() {
        let rect = Rectangle::new(0, 0, 100, 100);

        assert_eq!(rect.size(), &Size { width: 100, height: 100 });
    }

    #[test]
    fn test_origin() {
        let rect = Rectangle::new(10, 20, 100, 100);

        assert_eq!(rect.origin(), &Point { x: 10, y: 20 });
    }
}

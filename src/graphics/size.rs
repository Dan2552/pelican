pub struct Size<T> {
    pub width: T,
    pub height: T
}

impl<T> Clone for Size<T> where T: Copy {
    fn clone(&self) -> Self {
      Size {
        width: self.width,
        height: self.height
      }
    }
  }

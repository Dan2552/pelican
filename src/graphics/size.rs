pub struct Size {
    pub width: u32,
    pub height: u32
}

impl Clone for Size {
    fn clone(&self) -> Self {
      Size {
        width: self.width,
        height: self.height
      }
    }
  }

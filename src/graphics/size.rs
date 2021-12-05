use crate::graphics::Number;

pub struct Size<T> {
    pub width: T,
    pub height: T
}

impl<T> Size<T> where T: Number {
    pub fn new(width: T, height: T) -> Size<T> {
        Size {
            width: width,
            height: height
        }
    }
}

impl<T> Clone for Size<T> where T: Copy {
    fn clone(&self) -> Self {
        Size {
            width: self.width,
            height: self.height
        }
    }
}

impl<T> PartialEq for Size<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<T> std::fmt::Debug for Size<T> where T: Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Size")
        .field(&self.width)
        .field(&self.height)
        .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let size = Size {
            width: 1,
            height: 2
        };

        let size_clone = size.clone();

        assert_eq!(size, size_clone);
    }

    fn test_eq() {
        let size1 = Size {
            width: 1,
            height: 2
        };

        let size2 = Size {
            width: 1,
            height: 2
        };

        assert_eq!(size1, size2);
    }

    fn test_debug() {
        let size = Size {
            width: 1,
            height: 2
        };

        assert_eq!(format!("{:?}", size), "Size(1, 2)");
    }
}

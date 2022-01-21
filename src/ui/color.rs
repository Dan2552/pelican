use crate::graphics;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color { red, green, blue, alpha }
    }

    pub fn white() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255
        }
    }

    pub fn black() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255
        }
    }

    pub fn red() -> Color {
        Color {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255
        }
    }

    pub fn green() -> Color {
        Color {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255
        }
    }

    pub fn blue() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255
        }
    }

    pub fn gray() -> Color {
        Color {
            red: 128,
            green: 128,
            blue: 128,
            alpha: 255
        }
    }

    pub fn clear() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0
        }
    }

    pub fn to_graphics_color(&self) -> graphics::Color {
        graphics::Color {
            r: self.red,
            g: self.green,
            b: self.blue,
            a: self.alpha
        }
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Color")
         .field(&self.red)
         .field(&self.green)
         .field(&self.blue)
         .field(&self.alpha)
         .finish()
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Color) -> bool {
        self.red == rhs.red &&
            self.green == rhs.green &&
            self.blue == rhs.blue &&
            self.alpha == rhs.alpha
    }
}

impl Clone for Color {
    fn clone(&self) -> Color {
        Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_equality() {
        let color1 = Color::new(255, 255, 255, 255);
        let color2 = Color::new(255, 255, 255, 255);
        assert_eq!(color1, color2);

        let color1 = Color::new(255, 255, 255, 255);
        let color2 = Color::new(255, 255, 255, 0);
        assert_ne!(color1, color2);

        let color1 = Color::new(255, 255, 255, 255);
        let color2 = Color::new(0, 255, 255, 255);
        assert_ne!(color1, color2);
    }

    #[test]
    fn test_white() {
        let color = Color::white();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 255);
        assert_eq!(color.alpha, 255);
    }

    #[test]
    fn test_black() {
        let color = Color::black();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 255);
    }

    #[test]
    fn test_red() {
        let color = Color::red();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 255);
    }

    #[test]
    fn test_green() {
        let color = Color::green();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 255);
    }

    #[test]
    fn test_blue() {
        let color = Color::blue();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 255);
        assert_eq!(color.alpha, 255);
    }

    #[test]
    fn test_clear() {
        let color = Color::clear();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 0);
    }

    #[test]
    fn test_to_graphics_color() {
        let color = Color::new(123, 45, 67, 89);
        let graphics_color = color.to_graphics_color();
        assert_eq!(graphics_color.r, 123);
        assert_eq!(graphics_color.g, 45);
        assert_eq!(graphics_color.b, 67);
        assert_eq!(graphics_color.a, 89);
    }

    #[test]
    fn test_debug() {
        let color = Color::new(123, 45, 67, 89);
        assert_eq!(format!("{:?}", color), "Color(123, 45, 67, 89)");
    }
}

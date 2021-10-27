use crate::graphics;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl Color {
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

impl PartialEq for Color {
    fn eq(&self, rhs: &Color) -> bool {
        self.red == rhs.red &&
            self.green == rhs.green &&
            self.blue == rhs.blue &&
            self.alpha == rhs.alpha
    }
}

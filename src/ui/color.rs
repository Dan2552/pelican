pub struct Color {
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32
}

impl Color {
    fn white() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255
        }
    }

    fn black() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255
        }
    }

    fn red() -> Color {
        Color {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255
        }
    }

    fn green() -> Color {
        Color {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255
        }
    }

    fn blue() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255
        }
    }

    fn clear() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0
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

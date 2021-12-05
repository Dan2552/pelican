use crate::graphics::Color;
use crate::graphics::Font;
use std::cell::Ref;

enum Attribute {
    Color {
        color: Color
    }
}

pub struct AttributedString {
    text: String,
    attributes: Vec<Attribute>,
    default_font: Font
}

pub struct AttributedSubstring<'a> {
    attributed_string: &'a AttributedString,
    start: usize,
    end: usize
}

impl AttributedString {
    fn new(text: String) -> AttributedString {
        let default_font = Font::default();
        AttributedString {
            text: text,
            attributes: Vec::new(),
            default_font
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn lines(&self) -> Vec<AttributedSubstring> {
        let mut lines = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for (i, c) in self.text.chars().enumerate() {
            if c == '\n' {
                end = i;
                lines.push(AttributedSubstring {
                    attributed_string: self,
                    start: start,
                    end: end
                });
                start = i + 1;
            }
        }
        lines.push(AttributedSubstring {
            attributed_string: self,
            start: start,
            end: self.text.len()
        });
        lines
    }

    pub fn default_font(&self) -> &Font {
        &self.default_font
    }

    pub fn font_for(&self, index: usize) -> &Font {
        // TODO: unimplemented. Check all test cases reffering to this function.
        &self.default_font
    }

    pub fn chars(&self) -> std::str::Chars {
        // Get all characters within the range of the AttributedSubstring
        self.text.chars()
    }
}

impl AttributedSubstring<'_> {
    pub fn text(&self) -> &str {
        &self.attributed_string.text[self.start..self.end]
    }

    pub fn chars(&self) -> std::str::Chars {
        // Get all characters within the range of the AttributedSubstring
        self.text().chars()
    }

    pub fn font_for(&self, index: usize) -> &Font {
        self.attributed_string.font_for(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        assert_eq!(attributed_string.text(), text);
    }

    #[test]
    fn test_lines() {
        let text = "Hello, world!\nGoodbye, world!";
        let attributed_string = AttributedString::new(text.to_string());
        let lines = attributed_string.lines();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text(), "Hello, world!");
        assert_eq!(lines[1].text(), "Goodbye, world!");
    }

    #[test]
    fn test_default_font() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        assert_eq!(attributed_string.default_font(), &Font::default());
    }

    #[test]
    fn test_font_for() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        assert_eq!(attributed_string.font_for(0), &Font::default());
    }

    #[test]
    fn test_chars() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        assert_eq!(attributed_string.chars().count(), text.len());
        assert_eq!(attributed_string.chars().nth(0), Some('H'));
        assert_eq!(attributed_string.chars().nth(1), Some('e'));
        assert_eq!(attributed_string.chars().nth(2), Some('l'));
        assert_eq!(attributed_string.chars().nth(3), Some('l'));
        assert_eq!(attributed_string.chars().nth(4), Some('o'));
    }

    #[test]
    fn test_substring_text() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        let lines = attributed_string.lines();
        assert_eq!(lines[0].text(), "Hello, world!");
    }

    #[test]
    fn test_substring_chars() {
        let text = "Hello, world!\nGoodbye, world!";
        let attributed_string = AttributedString::new(text.to_string());
        let lines = attributed_string.lines();
        assert_eq!(lines[0].chars().count(), "Hello, world!".len());
        assert_eq!(lines[0].chars().nth(0), Some('H'));
        assert_eq!(lines[0].chars().nth(1), Some('e'));
        assert_eq!(lines[0].chars().nth(2), Some('l'));
        assert_eq!(lines[0].chars().nth(3), Some('l'));
        assert_eq!(lines[0].chars().nth(4), Some('o'));

        assert_eq!(lines[1].chars().count(), "Goodbye, world!".len());
        assert_eq!(lines[1].chars().nth(0), Some('G'));
        assert_eq!(lines[1].chars().nth(1), Some('o'));
        assert_eq!(lines[1].chars().nth(2), Some('o'));
        assert_eq!(lines[1].chars().nth(3), Some('d'));
        assert_eq!(lines[1].chars().nth(4), Some('b'));
        assert_eq!(lines[1].chars().nth(5), Some('y'));
        assert_eq!(lines[1].chars().nth(6), Some('e'));
    }

    #[test]
    fn test_substring_font_for() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        let lines = attributed_string.lines();
        assert_eq!(lines[0].font_for(0), &Font::default());
    }
}

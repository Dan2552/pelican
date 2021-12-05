use crate::graphics::Color;
use crate::graphics::Font;
use std::collections::HashMap;
use std::cell::{Ref, RefCell};


#[derive(PartialEq, Debug)]
pub enum Attribute {
    Color {
        color: Color
    },
    Font {
        font: Font
    }
}

impl Attribute {
    pub fn color(&self) -> &Color {
        match self {
            Attribute::Color { color } => color,
            _ => panic!("Attribute is not a color")
        }
    }

    pub fn font(&self) -> &Font {
        match self {
            Attribute::Font { font } => font,
            _ => panic!("Attribute is not a font")
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Key {
    Color,
    Font
}

type AttributeContainer = HashMap<Key, Attribute>;

pub struct AttributedString {
    /// The actual text that this `AttributedString` represents.
    text: String,

    /// The attributes for each character in the string. The index of the
    /// character in the string matches the index of the attribute in the
    /// vec.
    attributes: RefCell<Vec<AttributeContainer>>,

    /// The default attributes for the string.
    ///
    /// E.g. If any given character do not have the `Color` attribute, then
    /// the default color will be used.
    default_attributes: RefCell<AttributeContainer>
}

pub struct AttributedSubstring<'a> {
    attributed_string: &'a AttributedString,
    start: usize,
    end: usize
}

impl AttributedString {
    fn new(text: String) -> AttributedString {
        let mut default_attributes = AttributeContainer::new();
        default_attributes.insert(Key::Color, Attribute::Color { color: Color::BLACK });
        default_attributes.insert(Key::Font, Attribute::Font { font: Font::default() });

        let mut attributes = Vec::new();

        for _ in text.chars() {
            attributes.push(AttributeContainer::new());
        }

        AttributedString {
            text: text,
            attributes: RefCell::new(attributes),
            default_attributes: RefCell::new(default_attributes)
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn lines(&self) -> Vec<AttributedSubstring> {
        let mut lines = Vec::new();
        let mut start = 0;

        for (i, c) in self.text.chars().enumerate() {
            if c == '\n' {
                lines.push(AttributedSubstring {
                    attributed_string: self,
                    start: start,
                    end: i
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

    pub fn chars(&self) -> std::str::Chars {
        self.text.chars()
    }

    pub fn set_default_attribute(&self, key: Key, attribute: Attribute) {
        let mut default_attributes = self.default_attributes.borrow_mut();
        default_attributes.insert(key, attribute);
    }

    pub fn set_attribute_for(&self, index: usize, key: Key, attribute: Attribute) {
        let mut attributes = self.attributes.borrow_mut();

        if index >= self.text.len() {
            panic!("Index out of bounds");
        }

        attributes[index].insert(key, attribute);
    }

    pub fn get_attribute_for(&self, index: usize, key: Key) -> Ref<'_, Attribute> {
        let attributes = self.attributes.borrow();

        if index >= attributes.len() {
            panic!("Index out of bounds");
        }

        if attributes[index].get(&key).is_some() {
            Ref::map(attributes, |attrs| attrs[index].get(&key).unwrap())
        } else {
            let default_attributes = self.default_attributes.borrow();
            Ref::map(default_attributes, |attrs| attrs.get(&key).unwrap())
        }
    }
}

impl AttributedSubstring<'_> {
    pub fn text(&self) -> &str {
        &self.attributed_string.text[self.start..self.end]
    }

    pub fn chars(&self) -> std::str::Chars {
        self.text().chars()
    }

    pub fn set_attribute_for(&self, index: usize, key: Key, attribute: Attribute) {
        self.attributed_string.set_attribute_for(self.start + index, key, attribute);
    }

    pub fn get_attribute_for(&self, index: usize, key: Key) -> Ref<'_, Attribute> {
        self.attributed_string.get_attribute_for(self.start + index, key)
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
    fn test_set_default_attribute() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        attributed_string.set_default_attribute(Key::Color, Attribute::Color { color: Color::RED });
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED);
        assert_eq!(attributed_string.get_attribute_for(1, Key::Color).color(), &Color::RED);
    }

    #[test]
    fn test_set_attribute_for() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        attributed_string.set_attribute_for(0, Key::Color, Attribute::Color { color: Color::RED });
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED);
        assert_eq!(attributed_string.get_attribute_for(1, Key::Color).color(), &Color::BLACK);
    }

    #[test]
    fn test_get_attribute_for() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::BLACK);
        attributed_string.set_attribute_for(0, Key::Color, Attribute::Color { color: Color::RED });
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED);
    }

    #[test]
    fn test_substring_set_attribute_for() {
        let text = "Hello, world!\nGoodbye, world!";
        let attributed_string = AttributedString::new(text.to_string());
        attributed_string.set_attribute_for(0, Key::Color, Attribute::Color { color: Color::RED });
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED);

        // Test setting with the substring mutates both the substring and the original string
        let line0 = &attributed_string.lines()[0];
        line0.set_attribute_for(0, Key::Color, Attribute::Color { color: Color::BLUE });
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::BLUE);
        assert_eq!(line0.get_attribute_for(0, Key::Color).color(), &Color::BLUE);

        // Test mutating one line doesn't affect the other line
        let line1 = &attributed_string.lines()[1];
        line1.set_attribute_for(0, Key::Color, Attribute::Color { color: Color::GREEN });
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::BLUE);
        assert_eq!(line0.get_attribute_for(0, Key::Color).color(), &Color::BLUE);
        assert_eq!(line1.get_attribute_for(0, Key::Color).color(), &Color::GREEN);
    }
}

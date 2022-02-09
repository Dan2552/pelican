use crate::graphics::Color;
use crate::graphics::Font;
use std::collections::HashMap;
use std::cell::{Ref, RefCell};
use crate::text::text::Text;

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

impl Clone for Attribute {
    fn clone(&self) -> Attribute {
        match self {
            Attribute::Color { color } => Attribute::Color { color: color.clone() },
            Attribute::Font { font } => Attribute::Font { font: font.clone() }
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
    id: uuid::Uuid,

    /// The actual text that this `AttributedString` represents.
    text: Text,

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
    pub fn new(text: String) -> AttributedString {
        let mut default_attributes = AttributeContainer::new();
        default_attributes.insert(Key::Color, Attribute::Color { color: Color::BLACK });
        default_attributes.insert(Key::Font, Attribute::Font { font: Font::default() });

        let mut attributes = Vec::new();

        for _ in text.chars() {
            attributes.push(AttributeContainer::new());
        }

        AttributedString {
            id: uuid::Uuid::new_v4(),
            text: Text::new(text),
            attributes: RefCell::new(attributes),
            default_attributes: RefCell::new(default_attributes)
        }
    }

    pub fn new_matching_default_style(text: String, existing_attributed_string: &AttributedString) -> AttributedString {
        let attributed_string = AttributedString::new(text);

        let existing_color = existing_attributed_string.default_attributes.borrow().get(&Key::Color).unwrap().clone();
        attributed_string.set_default_attribute(Key::Color, existing_color);

        let existing_font = existing_attributed_string.default_attributes.borrow().get(&Key::Font).unwrap().clone();
        attributed_string.set_default_attribute(Key::Font, existing_font);

        attributed_string
    }

    pub fn text(&self) -> &Text {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.text.insert_str(idx, string);
        let text = Text::from(string);
        let mut attributes = self.attributes.borrow_mut();
        for _ in 0..text.len() {
            attributes.insert(idx, AttributeContainer::new())
        }
    }

    pub fn lines(&self) -> Vec<AttributedSubstring> {
        let mut lines = Vec::new();
        let mut start = 0;

        for (i, c) in self.text.string().chars().enumerate() {
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

    pub fn substring_for_char(&self, char_index: usize) -> AttributedSubstring {
        AttributedSubstring {
            attributed_string: self,
            start: char_index,
            end: char_index + 1
        }
    }

    pub fn chars(&self) -> std::str::Chars {
        self.text.string().chars()
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
            panic!("Index out of bounds. Attempted {}, but length is {} / {}", index, attributes.len(), self.text.string());
        }

        if attributes[index].get(&key).is_some() {
            Ref::map(attributes, |attrs| attrs[index].get(&key).unwrap())
        } else {
            self.default_attribute(key)
        }
    }

    pub fn default_attribute(&self, key: Key) -> Ref<'_, Attribute> {
        let default_attributes = self.default_attributes.borrow();
        Ref::map(default_attributes, |attrs| attrs.get(&key).unwrap())
    }

    pub fn replace_range(&mut self, range: std::ops::Range<usize>, string: &str) {
        let mut attributes = self.attributes.borrow_mut();
        let start = range.start;
        let end = range.end;

        self.text.replace_range(range, string);

        for _ in start..end {
            attributes.remove(start);
        }

        let text = Text::from(string);

        let mut new_attributes = Vec::new();
        for _ in 0..text.len() {
            new_attributes.push(AttributeContainer::new());
        }

        for i in start..start + text.len() {
            attributes.insert(i, new_attributes.remove(0));
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

    pub fn substring_for_char(&self, char_index: usize) -> AttributedSubstring {
        self.attributed_string.substring_for_char(self.start + char_index)
    }
}

impl std::fmt::Debug for AttributedString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AttributedString {{ text: \"{}\", attributes: [", self.text)?;
        let mut first = true;
        for attrs in self.attributes.borrow().iter() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{:?}", attrs)?;
        }
        write!(f, "] }}")
    }
}

impl PartialEq for AttributedString {
    fn eq(&self, other: &AttributedString) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let text = "Hello, world!";
        let attributed_string = AttributedString::new(text.to_string());
        assert_eq!(attributed_string.text(), &Text::from(text));
    }

    #[test]
    fn test_lines() {
        let text = "Hello, world!\nGoodbye, world!";
        let attributed_string = AttributedString::new(text.to_string());
        let lines = attributed_string.lines();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text(), "Hello, world!");
        assert_eq!(lines[1].text(), "Goodbye, world!");

        assert_eq!(lines[0].start, 0);
        assert_eq!(lines[0].end, 13);
        assert_eq!(lines[1].start, 14);
        assert_eq!(lines[1].end, 29);
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

    #[test]
    fn test_substring_for_char() {
        let text = "abc\ndef";
        let attributed_string = AttributedString::new(text.to_string());
        attributed_string.set_attribute_for(1, Key::Color, Attribute::Color { color: Color::RED });
        let substring = attributed_string.substring_for_char(1);
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::BLACK);
        assert_eq!(substring.get_attribute_for(0, Key::Color).color(), &Color::RED);

        let lines = attributed_string.lines();
        let substring = lines[1].substring_for_char(0);
        assert_eq!(substring.get_attribute_for(0, Key::Color).color(), &Color::BLACK);
        assert_eq!(substring.text(), "d");
    }

    #[test]
    fn test_insert_str() {
        let text = "three";
        let mut attributed_string = AttributedString::new(text.to_string());

        // Set "three" to red
        for i in 0..text.len() {
            attributed_string.set_attribute_for(i, Key::Color, Attribute::Color { color: Color::RED });
        };

        attributed_string.insert_str(0, "one ");
        assert_eq!(attributed_string.text(), &Text::from("one three"));
        attributed_string.insert_str(4, "two ");
        assert_eq!(attributed_string.text(), &Text::from("one two three"));

        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::BLACK);
        assert_eq!(attributed_string.get_attribute_for(1, Key::Color).color(), &Color::BLACK);
        assert_eq!(attributed_string.get_attribute_for(3, Key::Color).color(), &Color::BLACK);
        assert_eq!(attributed_string.get_attribute_for(4, Key::Color).color(), &Color::BLACK);
        assert_eq!(attributed_string.get_attribute_for(5, Key::Color).color(), &Color::BLACK);
        assert_eq!(attributed_string.get_attribute_for(6, Key::Color).color(), &Color::BLACK);
        assert_eq!(attributed_string.get_attribute_for(7, Key::Color).color(), &Color::BLACK);
        assert_eq!(attributed_string.get_attribute_for(8, Key::Color).color(), &Color::RED);
        assert_eq!(attributed_string.get_attribute_for(9, Key::Color).color(), &Color::RED);
        assert_eq!(attributed_string.get_attribute_for(10, Key::Color).color(), &Color::RED);
        assert_eq!(attributed_string.get_attribute_for(11, Key::Color).color(), &Color::RED);
        assert_eq!(attributed_string.get_attribute_for(12, Key::Color).color(), &Color::RED);
    }

    #[test]
    fn test_replace_range() {
        let text = "Hello, world!";
        let mut attributed_string = AttributedString::new(text.to_string());

        // Set "Hello" to red
        for i in 0..5 {
            attributed_string.set_attribute_for(i, Key::Color, Attribute::Color { color: Color::RED });
        };

        // Set "world" to blue
        for i in 7..12 {
            attributed_string.set_attribute_for(i, Key::Color, Attribute::Color { color: Color::BLUE });
        };

        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED); // H
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED); // e
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED); // l
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED); // l
        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::RED); // o
        assert_eq!(attributed_string.get_attribute_for(6, Key::Color).color(), &Color::BLACK); // ,
        assert_eq!(attributed_string.get_attribute_for(6, Key::Color).color(), &Color::BLACK); // " "
        assert_eq!(attributed_string.get_attribute_for(7, Key::Color).color(), &Color::BLUE); // w
        assert_eq!(attributed_string.get_attribute_for(8, Key::Color).color(), &Color::BLUE); // o
        assert_eq!(attributed_string.get_attribute_for(9, Key::Color).color(), &Color::BLUE); // r
        assert_eq!(attributed_string.get_attribute_for(10, Key::Color).color(), &Color::BLUE); // l
        assert_eq!(attributed_string.get_attribute_for(11, Key::Color).color(), &Color::BLUE); // d

        attributed_string.replace_range(0..5, "Goodbye");
        assert_eq!(attributed_string.text(), &Text::from("Goodbye, world!"));

        assert_eq!(attributed_string.get_attribute_for(0, Key::Color).color(), &Color::BLACK); // G
        assert_eq!(attributed_string.get_attribute_for(1, Key::Color).color(), &Color::BLACK); // o
        assert_eq!(attributed_string.get_attribute_for(2, Key::Color).color(), &Color::BLACK); // o
        assert_eq!(attributed_string.get_attribute_for(3, Key::Color).color(), &Color::BLACK); // d
        assert_eq!(attributed_string.get_attribute_for(4, Key::Color).color(), &Color::BLACK); // b
        assert_eq!(attributed_string.get_attribute_for(5, Key::Color).color(), &Color::BLACK); // y
        assert_eq!(attributed_string.get_attribute_for(6, Key::Color).color(), &Color::BLACK); // e
        assert_eq!(attributed_string.get_attribute_for(7, Key::Color).color(), &Color::BLACK); // ,
        assert_eq!(attributed_string.get_attribute_for(8, Key::Color).color(), &Color::BLACK); // " "
        assert_eq!(attributed_string.get_attribute_for(9, Key::Color).color(), &Color::BLUE);  // w
        assert_eq!(attributed_string.get_attribute_for(10, Key::Color).color(), &Color::BLUE); // o
        assert_eq!(attributed_string.get_attribute_for(11, Key::Color).color(), &Color::BLUE); // r
        assert_eq!(attributed_string.get_attribute_for(12, Key::Color).color(), &Color::BLUE); // l
        assert_eq!(attributed_string.get_attribute_for(13, Key::Color).color(), &Color::BLUE); // d
    }
}

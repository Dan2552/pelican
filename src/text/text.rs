use unicode_segmentation::UnicodeSegmentation;
use std::ops::Range;

/// A grapheme based structure for text. With intention to behave similarly to a
/// `String`.
///
/// Specifically, this struct is a wrapper around a `String` that upon
/// initialization will pre-index the grapheme indices of the string, so that
/// operations like finding a character at a given index will be O(1).
///
/// It's important to note that this isn't a magic universal fix for performance
/// with grapheme based strings, as the initialization is still O(n).
pub struct Text {
    string: String,
    grapheme_indices: Vec<usize>
}

impl Text {
    pub fn from(str: &str) -> Self {
        Self::new(String::from(str))
    }

    /// Creates a new `Text` from a `String`.
    ///
    /// This will pre-index the grapheme indices of the string, so that
    /// operations like finding a character at a given index will be O(1).
    ///
    /// This operation is O(n), where n is the length of the string.
    pub fn new(string: String) -> Self {
        let mut text = Text {
            string,
            grapheme_indices: Vec::new()
        };

        text.index();

        text
    }

    /// (Re)builds the known grapheme locations index of the string.
    fn index(&mut self) {
        let grapheme_indices = &mut self.grapheme_indices;
        grapheme_indices.clear();

        let mut index = 0;
        for character in self.string.graphemes(true) {
            grapheme_indices.push(index);
            index += character.len();
        }
    }

    /// Insert text from a str at the given index.
    ///
    /// This operation is O(n), where n is the length of the string.
    ///
    /// Future: This could be optimized to only update the indices after
    /// the insertion point.
    pub fn insert_str(&mut self, grapheme_index: usize, string: &str) {
        if grapheme_index == self.len() {
            self.string.push_str(string);
        } else {
            let string_index = self.grapheme_indices[grapheme_index];
            self.string.insert_str(string_index, string);
        }

        self.index();
    }

    /// Insert text from a `Text` at the given index.
    pub fn insert_text(&mut self, grapheme_index: usize, text: &Text) {
        self.insert_str(grapheme_index, &text.string);
    }

    /// Replace a range of text.
    pub fn replace_range(&mut self, range: Range<usize>, string: &str) {
        if range.end == self.len() {
            if range.start == self.len() {
                self.string.push_str(string);
            } else {
                let start_index = self.grapheme_indices[range.start];
                self.string.replace_range(start_index.., string);
            }
        } else {
            let start_index = self.grapheme_indices[range.start];
            let end_index = self.grapheme_indices[range.end];
            self.string.replace_range(start_index..end_index, string);
        }

        // Future: This could be optimized to only update the indices after
        // the insertion point.
        self.index();
    }

    /// Returns the count of graphemes in the text.
    ///
    /// This is O(1).
    pub fn len(&self) -> usize {
        self.grapheme_indices.len()
    }

    /// A reference to the underlying `String`.
    pub fn string(&self) -> &str {
        &self.string
    }

    /// Similar to `Index` but returns an `Option` instead of panicking when
    /// fetching out of bounds.
    pub fn nth(&self, index: usize) -> Option<&str> {
        if index >= self.len() {
            return None;
        }

        if self.grapheme_indices.len() == 0 {
            return Some(&self.string);
        }

        let start = self.grapheme_indices[index];
        if index + 1 == self.len() {
            Some(&self.string[start..])
        } else {
            let end = self.grapheme_indices[index + 1];
            Some(&self.string[start..end])
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl std::fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl std::ops::Index<Range<usize>> for Text {
    type Output = str;

    fn index(&self, index: Range<usize>) -> &str {
        if index.start == index.end {
            if index.start <= self.len() {
                return "";
            } else {
                panic!("character index {} is out of bounds of `{:?}`", index.start, self);
            }
        } else if index.end == self.len() {
            let start = self.grapheme_indices[index.start];
            &self.string[start..]
        } else {
            let start = self.grapheme_indices[index.start];
            let end = self.grapheme_indices[index.end];
            &self.string[start..end]
        }
    }
}

impl std::cmp::PartialEq for Text {
    fn eq(&self, other: &Text) -> bool {
        self.string == other.string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let text = Text::new("hello world".to_string());
        assert_eq!(text.len(), 11);
        assert_eq!(text.string().len(), 11);
        assert_eq!(text.string().chars().count(), 11);

        let text = Text::new("é".to_string());
        assert_eq!(text.len(), 1);
        assert_eq!(text.string().len(), 2);
        assert_eq!(text.string().chars().count(), 1);

        let text = Text::new("👨‍👨‍👧‍👧".to_string());
        assert_eq!(text.len(), 1);
        assert_eq!(text.string().len(), 25);
        assert_eq!(text.string().chars().count(), 7);

        let text = Text::new("".to_string());
        assert_eq!(text.len(), 0);
    }

    #[test]
    fn test_insert_str() {
        let mut text = Text::new("hello world".to_string());
        text.insert_str(5, "!");
        assert_eq!(text.string(), "hello! world");
        assert_eq!(text.len(), 12);
        assert_eq!(text.string().len(), 12);
        assert_eq!(text.string().chars().count(), 12);

        let mut text = Text::new("é".to_string());
        text.insert_str(0, "hello");
        assert_eq!(text.string(), "helloé");
        assert_eq!(text.len(), 6);
        assert_eq!(text.string().len(), 7);
        assert_eq!(text.string().chars().count(), 6);

        let mut text = Text::new("👨‍👨‍👧‍👧".to_string());
        text.insert_str(0, "hello");
        assert_eq!(text.string(), "hello👨‍👨‍👧‍👧");
        assert_eq!(text.len(), 6);
        assert_eq!(text.string().len(), 30);
        assert_eq!(text.string().chars().count(), 12);

        let mut text = Text::new("hello".to_string());
        text.insert_str(2, "é");
        assert_eq!(text.string(), "heéllo");
        assert_eq!(text.len(), 6);
        assert_eq!(text.nth(0).unwrap(), "h");
        assert_eq!(text.nth(1).unwrap(), "e");
        assert_eq!(text.nth(2).unwrap(), "é");
        assert_eq!(text.nth(3).unwrap(), "l");
        assert_eq!(text.nth(4).unwrap(), "l");
        assert_eq!(text.nth(5).unwrap(), "o");

        let mut text = Text::new("hello".to_string());
        text.insert_str(5, " world!");
        assert_eq!(text.len(), 12);

        let mut text = Text::new("👩‍👩‍👦".to_string());
        text.insert_str(1, " hello");
        assert_eq!(text.len(), 7);

        let mut text = Text::new("".to_string());
        text.insert_str(0, "hello");
        assert_eq!(text.len(), 5);
    }

    #[test]
    fn test_insert_text() {
        let mut text = Text::new("hello world".to_string());
        text.insert_text(5, &Text::from("!"));
        assert_eq!(text.string(), "hello! world");
        assert_eq!(text.len(), 12);
        assert_eq!(text.string().len(), 12);
        assert_eq!(text.string().chars().count(), 12);

        let mut text = Text::new("é".to_string());
        text.insert_text(0, &Text::from("hello"));
        assert_eq!(text.string(), "helloé");
        assert_eq!(text.len(), 6);
        assert_eq!(text.string().len(), 7);
        assert_eq!(text.string().chars().count(), 6);

        let mut text = Text::new("👨‍👨‍👧‍👧".to_string());
        text.insert_text(0, &Text::from("hello"));
        assert_eq!(text.string(), "hello👨‍👨‍👧‍👧");
        assert_eq!(text.len(), 6);
        assert_eq!(text.string().len(), 30);
        assert_eq!(text.string().chars().count(), 12);

        let mut text = Text::new("hello".to_string());
        text.insert_text(2, &Text::from("é"));
        assert_eq!(text.string(), "heéllo");
        assert_eq!(text.len(), 6);
        assert_eq!(text.nth(0).unwrap(), "h");
        assert_eq!(text.nth(1).unwrap(), "e");
        assert_eq!(text.nth(2).unwrap(), "é");
        assert_eq!(text.nth(3).unwrap(), "l");
        assert_eq!(text.nth(4).unwrap(), "l");
        assert_eq!(text.nth(5).unwrap(), "o");

        let mut text = Text::new("hello".to_string());
        text.insert_text(5, &Text::from(" world!"));
        assert_eq!(text.len(), 12);

        let mut text = Text::new("👩‍👩‍👦".to_string());
        text.insert_text(1, &Text::from(" hello"));
        assert_eq!(text.len(), 7);

        let mut text = Text::new("".to_string());
        text.insert_text(0, &Text::from("hello"));
        assert_eq!(text.len(), 5);
    }

    #[test]
    fn test_replace_range() {
        let mut text = Text::new("hello world".to_string());
        text.replace_range(5..11, "!");
        assert_eq!(text.string(), "hello!");
        assert_eq!(text.len(), 6);
        assert_eq!(text.string().len(), 6);
        assert_eq!(text.string().chars().count(), 6);

        let mut text = Text::new("héllo world".to_string());
        text.replace_range(5..11, "!");
        assert_eq!(text.string(), "héllo!");
        assert_eq!(text.len(), 6);

        let mut text = Text::new("👨‍👨‍👧‍👧".to_string());
        text.replace_range(0..0, "hello");
        assert_eq!(text.string(), "hello👨‍👨‍👧‍👧");
        assert_eq!(text.len(), 6);

        let mut text = Text::new("👨‍👨‍👧‍👧".to_string());
        text.replace_range(0..1, "");
        assert_eq!(text.string(), "");
        assert_eq!(text.len(), 0);

        let mut text = Text::new("a".to_string());
        text.replace_range(1..1, "hello");
        assert_eq!(text.string(), "ahello");

        let mut text = Text::new("👨‍👨‍👧‍👧".to_string());
        text.replace_range(1..1, "hello");
        assert_eq!(text.string(), "👨‍👨‍👧‍👧hello");
        assert_eq!(text.len(), 6);

        let mut text = Text::new("".to_string());
        text.replace_range(0..0, "hello");
        assert_eq!(text.string(), "hello");
        assert_eq!(text.len(), 5);
    }

    #[test]
    fn test_display() {
        let text = Text::new("hello world".to_string());
        assert_eq!(text.to_string(), "hello world");
    }

    #[test]
    fn test_debug() {
        let text = Text::new("hello world".to_string());
        assert_eq!(format!("{:?}", text), "hello world");
    }

    #[test]
    fn test_index() {
        let text = Text::new("hello world".to_string());
        assert_eq!(text.nth(0).unwrap(), "h");
        assert_eq!(text.nth(1).unwrap(), "e");
        assert_eq!(text.nth(2).unwrap(), "l");
        assert_eq!(text.nth(3).unwrap(), "l");
        assert_eq!(text.nth(4).unwrap(), "o");
        assert_eq!(text.nth(5).unwrap(), " ");
        assert_eq!(text.nth(6).unwrap(), "w");
        assert_eq!(text.nth(7).unwrap(), "o");
        assert_eq!(text.nth(8).unwrap(), "r");
        assert_eq!(text.nth(9).unwrap(), "l");
        assert_eq!(text.nth(10).unwrap(), "d");

        let text = Text::new("héllo".to_string());
        assert_eq!(text.nth(0).unwrap(), "h");
        assert_eq!(text.nth(1).unwrap(), "é");
        assert_eq!(text.nth(2).unwrap(), "l");
        assert_eq!(text.nth(3).unwrap(), "l");
        assert_eq!(text.nth(4).unwrap(), "o");

        let text = Text::new("a👨‍👨‍👧‍👧b".to_string());
        assert_eq!(text.nth(0).unwrap(), "a");
        assert_eq!(text.nth(1).unwrap(), "👨‍👨‍👧‍👧");
        assert_eq!(text.nth(2).unwrap(), "b");

        let text = Text::new("".to_string());
        assert!(text.nth(0).is_none());
    }

    #[test]
    fn test_index_with_range() {
        let text = Text::new("hello world".to_string());
        assert_eq!(&text[0..5], "hello");
        assert_eq!(&text[0..6], "hello ");
        assert_eq!(&text[0..10], "hello worl");
        assert_eq!(&text[0..11], "hello world");

        let text = Text::new("héllo".to_string());
        assert_eq!(&text[0..2], "hé");
        assert_eq!(&text[2..5], "llo");


        let text = Text::new("".to_string());
        assert_eq!(&text.string()[0..0], "");
        assert_eq!(&text[0..0], "");
    }

    #[test]
    fn test_range_empty() {
        let string = "";
        assert_eq!(&string[0..0], "");

        let text = Text::new("".to_string());
        assert_eq!(&text[0..0], "");
    }

    #[test]
    #[should_panic(expected = "byte index 1 is out of bounds of ``")]
    fn test_range_edge_case_proof() {
        let string = "";
        let _ = &string[1..1];
    }
    #[test]
    #[should_panic(expected = "character index 1 is out of bounds of ``")]
    fn test_range_edge_case_actual() {
        let text = Text::new("".to_string());
        let _ = &text[1..1];
    }

    #[test]
    fn test_range_same() {
        let string = "a";
        let text = Text::new("a".to_string());
        assert_eq!(&string[0..0], "");
        assert_eq!(&text[0..0], "");
    }

    #[test]
    fn test_newline() {
        let text = Text::new("\n".to_string());
        assert_eq!(text.len(), 1);
        assert_eq!(text.string(), "\n");
        assert_eq!(text.nth(0), Some("\n"));
        assert_eq!(text.nth(1), None);
        assert_eq!(&text[0..1], "\n");
    }

    #[test]
    fn test_equal() {
        let a = Text::from("hello world");
        let b = Text::from("hello world");

        assert_eq!(a, b);

        let c = Text::from("different");

        assert_ne!(a, c);
    }
}

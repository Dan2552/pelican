use crate::graphics::Rectangle;
use crate::graphics::Point;
use crate::graphics::Size;
use crate::text::attributed_string::{AttributedString, AttributedSubstring};
use crate::text::attributed_string;

/// Used for rendering.
///
/// A character to be rendered as part of a `Word`.
///
/// The `Character` struct itself doesn't contain any information about the
/// position. This is because the `Word` struct will ensure following characters
/// are always be sequential / on the same line.
pub struct Character {
    /// The character to be rendered.
    character: char,

    /// The size of the character.
    size: Size<u32>
}

/// Used for rendering.
///
/// A word as part of `LineOfText`. A word is delimited by spaces.
///
/// A word's characters can be of varying styles and sizes.
///
/// If an individual word is too long to fit on a line, it will be broken up
/// into multiple `Word`s for simplicity of rendering.
pub struct Word {
    text: Vec<Character>,

    /// We only care about the x position of the character, because the y
    /// position is determined by the line height (so is up to `LineOfText`).
    x: i32,

    /// The size of the word.
    size: Size<u32>
}

/// Used for rendering.
///
/// A whole line of text, containing `Word`s.
pub struct LineOfText {
    /// The text to be rendered.
    words: Vec<Word>,

    /// The size of the line.
    ///
    /// The height of the line is determined by the largest character in the
    /// line.
    ///
    /// The width of the line is determined by the sum of the widths of all
    /// characters in the line.
    size: Size<u32>
}

/// Used for rendering.
///
/// A paragraph (or many paragraphs) of text, containing `LineOfText`s.
pub struct WholeText {
    /// The text to be rendered.
    text: Vec<LineOfText>,

    /// The position and size of the text.
    ///
    /// The height of the text is determined by the sum of the heights of all
    /// lines.
    ///
    /// The width of the text is determined by the largest line.
    frame: Rectangle<i32, u32>
}

impl Character {
    fn is_whitespace(&self) -> bool {
        self.character.is_whitespace()
    }
}

impl std::fmt::Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.character)
    }
}

impl Word {
    /// Constructs an empty `Word`, to be populated with `add_character`.
    fn new(x: i32) -> Word {
        Word {
            text: Vec::new(),
            x: x,
            size: Size::new(0, 0)
        }
    }

    /// Returns whether the word is empty (i.e. there are no characters).
    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Add a character to the word. This will update the word's size.
    ///
    /// Returns a borrow reference to the character that was added.
    fn add_character(&mut self, character: Character) -> &Character{
        self.size.width += character.size.width;
        self.size.height = character.size.height.max(self.size.height);
        self.text.push(character);
        self.text.last().unwrap()
    }

    /// Set the x position of the word. I.e. for use when the word is wrapped
    /// to a new line.
    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Calculate the x of the next word (the word following this one).
    fn next_word_x(&self) -> i32 {
        let width = self.size.width as i32;
        self.x + width
    }
}

impl std::fmt::Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Word({})", self.text.iter().map(|c| c.character).collect::<String>())
    }
}

impl LineOfText {
    /// Creates a `LineOfText` from an `AttributedSubstring`.
    ///
    /// The passed in `AttributedSubstring` is assumed to be a single line of
    /// text. For handling multiple lines, use `WholeText`.
    ///
    /// However, the output can still be more than one `LineOfText` if the text
    /// needs to word-wrap.
    fn from(attributed_string: &AttributedSubstring, maximum_width: u32) -> Vec<LineOfText> {
        // The output.
        let mut lines = Vec::new();

        // The words in the current line. Words will only be inserted once the
        // whole word is formed. If a word needs to be wrapped, it will cause
        // a new line to be created.
        let mut current_line_words: Vec<Word> = Vec::new();

        // The current width of all words already contained in
        // `current_line_words`.
        let mut current_line_width = 0;

        // The current (maximum) height of all words already contained in
        // `current_line_words`.
        let mut current_line_height = 0;

        // The current word being formed.
        let mut current_word = Word::new(0);

        for (char_index, character) in attributed_string.chars().enumerate() {
            // Calculate the size of the character.
            let font_attribute = attributed_string
                .get_attribute_for(char_index, attributed_string::Key::Font);
            let font = font_attribute.font();
            let size = font.size_for(&String::from(character));
            let character = Character { character, size };

            // Add the character to the current word.
            let character = current_word.add_character(character);

            // If the character is whitespace, it's added to the current word,
            // but specifically we don't care if it fits.
            //
            // If the character is not whitespace, we need to check if the
            // word will still fit on the current line, or if the word needs to
            // be wrapped.
            if character.is_whitespace() {
                // Add the current word to the current line.
                current_line_width += current_word.size.width;
                let next_word_x = current_word.next_word_x();
                current_line_height = current_word.size.height.max(current_line_height);
                current_line_words.push(current_word);

                // Reset the current word.
                current_word = Word::new(next_word_x);
            } else {
                // If the word won't fit on the current line, we need to
                // wrap it.
                if current_line_width + current_word.size.width > maximum_width {
                    if current_line_words.is_empty() {
                        // If the current line is empty, we can't wrap it.
                        //
                        // This can happen if the first word is too long to fit
                        // on the first line.
                        panic!("First word is too long to fit on the first line.");
                    }
                    // Add the current line to the output.
                    lines.push(LineOfText {
                        words: current_line_words,
                        size: Size::new(current_line_width, current_line_height)
                    });

                    // Reset the current line.
                    current_line_words = Vec::new();
                    current_line_width = 0;
                    current_line_height = 0;

                    // Reset the x of the current word.
                    current_word.set_x(0);
                }
            }
        }

        // If there's still a word left, add it to the current line.
        if !current_word.is_empty() {
            current_line_width += current_word.size.width;
            current_line_height = current_word.size.height.max(current_line_height);
            current_line_words.push(current_word);
        }

        // If there are words on the remaining line, add it to the output.
        if !current_line_words.is_empty() {
            lines.push(LineOfText {
                words: current_line_words,
                size: Size::new(current_line_width, current_line_height)
            });
        }

        lines
    }
}

impl std::fmt::Debug for LineOfText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LineOfText({:?})", self.words)
    }
}

impl WholeText {
    // / Creates a `rendering::WholeText` from an `AttributedString`.
    // /
    // / The frame is used for:
    // / * Positioning the text where desired to be rendered
    // / * Affecting how long lines can be, and whether they word-wrap.
    // /
    // / The height of the frame is ignored; these structs do not deal with any
    // / truncation rules. Therefore the resulting `WholeText` may be taller than
    // / the frame supplied.
    // pub fn from(attributed_string: &AttributedString, frame: Rectangle<i32, u32>) -> WholeText {
        // let mut lines: Vec<LineOfText> = Vec::new();

        // for line in attributed_string.lines() {
        //     let y = if let Some(previous_line) = lines.last() {
        //         previous_line.frame.bottom()
        //     } else {
        //         0
        //     };

        //     // TODO: this will be updated as part of alignment handling.
        //     let x = frame.origin.x;

        //     let origin_of_line = Point::new(x, y);

        //     for line_of_text in LineOfText::from(&line, origin_of_line, frame.size.width) {
        //         lines.push(line_of_text);
        //     }
        // }

        // // Build a new frame for the text because the height of the text may
        // // not match the passed in frame.
        // let height = lines.iter().map(|line| line.frame.size.height).sum();
        // let width = frame.size.width;
        // let size = Size::new(width, height);
        // let frame = Rectangle { origin: frame.origin, size };

        // WholeText {
        //     text: lines,
        //     frame
        // }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        let character = Character {
            character: 'a',
            size: Size::new(10, 20)
        };

        assert_eq!(character.character, 'a');
        assert_eq!(character.size, Size::new(10, 20));
    }

    #[test]
    fn test_word() {
        let x = 10;
        let mut word = Word::new(x);

        word.add_character(Character {
            character: 'a',
            size: Size::new(10, 20)
        });

        assert_eq!(word.text.len(), 1);
        assert_eq!(word.text[0].character, 'a');
        assert_eq!(word.text[0].size, Size::new(10, 20));
        assert_eq!(word.size, Size::new(10, 20));
        assert_eq!(word.x, 10);

        let next_word_x = word.next_word_x();
        assert_eq!(next_word_x, 20);
    }

    #[test]
    fn test_word_is_empty() {
        let mut word = Word::new(0);
        assert!(word.is_empty());

        word.add_character(Character {
            character: 'a',
            size: Size::new(10, 20)
        });

        assert!(!word.is_empty());
    }

    #[test]
    fn test_word_set_x() {
        let mut word = Word::new(0);
        assert_eq!(word.x, 0);
        word.set_x(10);
        assert_eq!(word.x, 10);
    }

    #[test]
    fn test_line_of_text() {
        let attributed_string = AttributedString::new(String::from("Hello, world!"));
        let lines = attributed_string.lines();
        let line = lines.first().unwrap();

        let lines_of_text = LineOfText::from(&line, 100);
        assert_eq!(lines_of_text.len(), 1);
        let line_of_text = &lines_of_text[0];

        assert_eq!(line_of_text.words.len(), 2);
        assert_eq!(line_of_text.size, Size::new(90, 16));

        let word1 = &line_of_text.words[0];
        let word2 = &line_of_text.words[1];

        assert_eq!(word1.text[0].character, 'H');
        assert_eq!(word1.text[1].character, 'e');
        assert_eq!(word1.text[2].character, 'l');
        assert_eq!(word1.text[3].character, 'l');
        assert_eq!(word1.text[4].character, 'o');
        assert_eq!(word1.text[5].character, ',');
        assert_eq!(word1.text[6].character, ' ');
        assert_eq!(word2.text[0].character, 'w');
        assert_eq!(word2.text[1].character, 'o');
        assert_eq!(word2.text[2].character, 'r');
        assert_eq!(word2.text[3].character, 'l');
        assert_eq!(word2.text[4].character, 'd');
        assert_eq!(word2.text[5].character, '!');

        assert_eq!(word1.size, Size::new(46, 16));
        assert_eq!(word2.size, Size::new(44, 16));

        assert_eq!(line_of_text.size, Size::new(90, 16));
    }

    #[test]
    fn test_line_of_text_with_word_wrap() {
        let attributed_string = AttributedString::new(String::from("Hello, world!"));
        let lines = attributed_string.lines();
        let line = lines.first().unwrap();

        let lines_of_text = LineOfText::from(&line, 50);
        assert_eq!(lines_of_text.len(), 2);

        let line1 = &lines_of_text[0];
        let line2 = &lines_of_text[1];

        assert_eq!(format!("{:?}", line1), "LineOfText([Word(Hello, )])");
        assert_eq!(format!("{:?}", line2), "LineOfText([Word(world!)])");

        assert_eq!(line1.size, Size::new(46, 16));
        assert_eq!(line2.size, Size::new(44, 16));
    }

    #[test]
    fn test_line_of_text_with_single_word_that_wraps() {
        let attributed_string = AttributedString::new(String::from("Hello, world!"));
        let lines = attributed_string.lines();
        let line = lines.first().unwrap();

        let lines_of_text = LineOfText::from(&line, 5);
        println!("{:?}", lines_of_text);

        assert_eq!(lines_of_text.len(), 2);

        // let line1 = &lines_of_text[0];
        // let line2 = &lines_of_text[1];

        // assert_eq!(format!("{:?}", line1), "LineOfText([Word(Hello)])");
        // assert_eq!(format!("{:?}", line2), "LineOfText([Word(world!)])");

        // assert_eq!(line1.size, Size::new(46, 16));
        // assert_eq!(line2.size, Size::new(44, 16));
    }
}

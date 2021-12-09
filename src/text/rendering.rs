use crate::graphics::Rectangle;
use crate::graphics::Point;
use crate::graphics::Size;
use crate::text::attributed_string::{AttributedString, AttributedSubstring};
use crate::text::attributed_string;
use crate::text::{VerticalAlignment, HorizontalAlignment};

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
    size: Size<u32>,
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
    characters: Vec<Character>,

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
///
/// The `WholeText` struct contains lines of text to be rendered, but also
/// the positions for them. This is designed so that the `WholeText` has total
/// control for text-alignment.
pub struct WholeText<'a> {
    /// The text to be rendered.
    lines: Vec<LineOfText>,

    /// Positions of each line of text.
    ///
    /// Note that these positions are relative to the top-left of the
    /// `WholeText`.
    positions: Vec<Point<i32>>,

    /// The position and size of the text.
    ///
    /// The height of the text is determined by the sum of the heights of all
    /// lines.
    ///
    /// The width of the text is determined by the largest line.
    frame: Rectangle<i32, u32>,

    /// The attributed string that was used to create this `WholeText`.
    /// Contains the styles and sizes of each character.
    attributed_string: &'a AttributedString
}

impl Character {
    fn is_whitespace(&self) -> bool {
        self.character.is_whitespace()
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    pub fn to_string(&self) -> String {
        self.character.to_string()
    }
}

impl std::fmt::Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.character)
    }
}

impl Word {
    /// Constructs an empty `Word`, to be populated with `add_character`.
    fn new() -> Word {
        Word {
            characters: Vec::new(),
            size: Size::new(0, 0)
        }
    }

    /// Returns whether the word is empty (i.e. there are no characters).
    fn is_empty(&self) -> bool {
        self.characters.is_empty()
    }

    /// Add a character to the word. This will update the word's size.
    ///
    /// Returns a borrow reference to the character that was added.
    fn add_character(&mut self, character: Character) -> &Character{
        self.size.width += character.size.width;
        self.size.height = character.size.height.max(self.size.height);
        self.characters.push(character);
        self.characters.last().unwrap()
    }
}

impl std::fmt::Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Word({})", self.characters.iter().map(|c| c.character).collect::<String>())
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
    fn from(attributed_string: &AttributedSubstring, maximum_width: u32, render_scale: f32) -> Vec<LineOfText> {
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
        let mut current_word = Word::new();

        for (char_index, character) in attributed_string.chars().enumerate() {
            // Calculate the size of the character.
            let font_attribute = attributed_string
                .get_attribute_for(char_index, attributed_string::Key::Font);
            let font = font_attribute.font();
            let size = font.size_for(&String::from(character));
            let size = Size::new(
                (size.width as f32 * render_scale) as u32,
                (size.height as f32 * render_scale) as u32
            );

            let character = Character { character, size };

            let potential_word_width = current_word.size.width + character.size.width;
            if potential_word_width > maximum_width {
                // The word is too long to fit on the current line so must be
                // broken up.
                if !current_word.is_empty() {
                    current_line_width += current_word.size.width;
                    current_line_height = current_word.size.height.max(current_line_height);
                    current_line_words.push(current_word);
                }

                if !current_line_words.is_empty() {
                    // Add the current line to the output.
                    lines.push(LineOfText {
                        words: current_line_words,
                        size: Size::new(current_line_width, current_line_height)
                    });
                }

                // Reset the current line.
                current_line_words = Vec::new();
                current_line_width = 0;
                current_line_height = 0;

                // Reset the current word.
                current_word = Word::new();
            }

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
                current_line_height = current_word.size.height.max(current_line_height);
                current_line_words.push(current_word);

                // Reset the current word.
                current_word = Word::new();
            } else {
                // If the word won't fit on the current line, we need to
                // wrap it.
                if current_line_width + current_word.size.width > maximum_width {
                    // Add the current line to the output.
                    if !current_line_words.is_empty() {
                        lines.push(LineOfText {
                            words: current_line_words,
                            size: Size::new(current_line_width, current_line_height)
                        });
                    }

                    // Reset the current line.
                    current_line_words = Vec::new();
                    current_line_width = 0;
                    current_line_height = 0;
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

    /// The size of the line.
    ///
    /// However, if the last character in the line is whitespace, it will not
    /// be included in the size. This is because if the line wraps, we don't
    /// want it to affect the alignment (center or right) of the line.
    fn visual_size(&self) -> Size<u32> {
        if let Some(last_word) = self.words.last() {
            if let Some(last_character) = last_word.characters.last() {
                if last_character.is_whitespace() {
                    return Size {
                        width: self.size.width - last_character.size.width,
                        height: self.size.height
                    };
                }
            }
        }

        self.size.clone()
    }
}

impl std::fmt::Debug for LineOfText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LineOfText({:?})", self.words)
    }
}

impl WholeText<'_> {
    /// Creates a `rendering::WholeText` from an `AttributedString`.
    pub fn from(attributed_string: &AttributedString, frame: Rectangle<i32, u32>, render_scale: f32) -> WholeText {
        let frame = &frame * render_scale;
        let mut lines: Vec<LineOfText> = Vec::new();
        let mut positions = Vec::new();

        // Build the lines of text. The size of the lines is calculated during
        // this process.
        for line in attributed_string.lines() {
            for line_of_text in LineOfText::from(&line, frame.size.width, render_scale) {
                lines.push(line_of_text);
                positions.push(Point { x: 0, y: 0 });
            }
        }

        // Knowing the size of the lines, we can calculate positions based on
        // the text alignment rules.
        let mut whole_text = WholeText { lines, positions, frame, attributed_string };

        whole_text.align_horizontally(HorizontalAlignment::Left);
        whole_text.align_vertically(VerticalAlignment::Top);

        whole_text
    }

    fn lines_total_height(&self) -> u32 {
        self.lines.iter().fold(0, |acc, line| acc + line.size.height)
    }

    pub fn align_horizontally(&mut self, horizontal_alignment: HorizontalAlignment) {
        // Aligning horizontalling is simple as we don't need to account for
        // other lines of text, as they cannot overlap horizontally.
        for (index, line) in self.lines.iter().enumerate() {
            match horizontal_alignment {
                HorizontalAlignment::Left => {
                    self.positions[index].x = 0;
                }
                HorizontalAlignment::Center => {
                    let center_x = self.frame.size.width as f32 * 0.5;
                    let center_line_x = line.visual_size().width as f32 * 0.5;
                    let top_left_x = center_x - center_line_x;

                    self.positions[index].x = top_left_x.round() as i32;
                }
                HorizontalAlignment::Right => {
                    self.positions[index].x = (self.frame.size.width - line.visual_size().width) as i32;
                }
            }
        }
    }

    pub fn align_vertically(&mut self, vertical_alignment: VerticalAlignment) {
        // Aligning vertically is a bit more complicated as we need to account
        // for other lines of text.
        match vertical_alignment {
            VerticalAlignment::Top => {
                let mut line_y = 0;
                for (index, line) in self.lines.iter().enumerate() {
                    self.positions[index].y = line_y;
                    line_y += line.size.height as i32;
                }
            }
            VerticalAlignment::Middle => {
                let middle_y = self.frame.size.height as f32 * 0.5;
                let middle_line_y = self.lines_total_height() as f32 * 0.5;
                let top_left_y = middle_y - middle_line_y;

                let mut line_y = top_left_y.round() as i32;
                for (index, line) in self.lines.iter().enumerate() {
                    self.positions[index].y = line_y;
                    line_y += line.size.height as i32;
                }
            }
            VerticalAlignment::Bottom => {
                let bottom_y = self.frame.size.height as i32 - self.lines_total_height() as i32;
                let mut line_y = bottom_y;
                for (index, line) in self.lines.iter().enumerate() {
                    self.positions[index].y = line_y;
                    line_y += line.size.height as i32;
                }
            }
        }
    }

    /// Iterate chars with their positions.
    pub fn iter_characters_with_position(&self) -> impl Iterator<Item = (&Character, Point<i32>)> {
        self.lines.iter().enumerate().flat_map(move |(line_index, line)| {
            let line_relative_position = &self.positions[line_index];

            let mut word_x = 0;
            line.words.iter().flat_map(move |word| {
                let word_relative_position = Point {
                    x: word_x + line_relative_position.x,
                    y: line_relative_position.y
                };

                word_x += word.size.width as i32;

                let mut character_x = 0;
                word.characters.iter().map(move |character| {
                    let character_relative_position = Point {
                        x: character_x + word_relative_position.x,
                        y: word_relative_position.y
                    };

                    character_x += character.size.width as i32;

                    let absolute_position = Point {
                        x: character_relative_position.x,
                        y: character_relative_position.y
                    };

                    (character, absolute_position)
                })
            })
        })
    }
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
        assert_eq!(character.size().width, 10);
        assert_eq!(character.size().height, 20);
    }

    #[test]
    fn test_word() {
        let mut word = Word::new();

        word.add_character(Character {
            character: 'a',
            size: Size::new(10, 20)
        });

        assert_eq!(word.characters.len(), 1);
        assert_eq!(word.characters[0].character, 'a');
        assert_eq!(word.characters[0].size, Size::new(10, 20));
        assert_eq!(word.size, Size::new(10, 20));
    }

    #[test]
    fn test_word_is_empty() {
        let mut word = Word::new();
        assert!(word.is_empty());

        word.add_character(Character {
            character: 'a',
            size: Size::new(10, 20)
        });

        assert!(!word.is_empty());
    }

    #[test]
    fn test_line_of_text() {
        let attributed_string = AttributedString::new(String::from("Hello, world!"));
        let lines = attributed_string.lines();
        let line = lines.first().unwrap();

        let lines_of_text = LineOfText::from(&line, 100, 1.0);
        assert_eq!(lines_of_text.len(), 1);
        let line_of_text = &lines_of_text[0];

        assert_eq!(line_of_text.words.len(), 2);
        assert_eq!(line_of_text.size, Size::new(90, 16));

        let word1 = &line_of_text.words[0];
        let word2 = &line_of_text.words[1];

        assert_eq!(word1.characters[0].character, 'H');
        assert_eq!(word1.characters[1].character, 'e');
        assert_eq!(word1.characters[2].character, 'l');
        assert_eq!(word1.characters[3].character, 'l');
        assert_eq!(word1.characters[4].character, 'o');
        assert_eq!(word1.characters[5].character, ',');
        assert_eq!(word1.characters[6].character, ' ');
        assert_eq!(word2.characters[0].character, 'w');
        assert_eq!(word2.characters[1].character, 'o');
        assert_eq!(word2.characters[2].character, 'r');
        assert_eq!(word2.characters[3].character, 'l');
        assert_eq!(word2.characters[4].character, 'd');
        assert_eq!(word2.characters[5].character, '!');

        assert_eq!(word1.size, Size::new(46, 16));
        assert_eq!(word2.size, Size::new(44, 16));

        assert_eq!(line_of_text.size, Size::new(90, 16));
    }

    #[test]
    fn test_line_of_text_with_word_wrap() {
        let attributed_string = AttributedString::new(String::from("Hello, world!"));
        let lines = attributed_string.lines();
        let line = lines.first().unwrap();

        let lines_of_text = LineOfText::from(&line, 50, 1.0);
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

        let lines_of_text = LineOfText::from(&line, 35, 1.0);

        assert_eq!(lines_of_text.len(), 4);

        let line1 = &lines_of_text[0];
        let line2 = &lines_of_text[1];
        let line3 = &lines_of_text[2];
        let line4 = &lines_of_text[3];

        assert_eq!(format!("{:?}", line1), "LineOfText([Word(Hell)])");
        assert_eq!(format!("{:?}", line2), "LineOfText([Word(o, )])");
        assert_eq!(format!("{:?}", line3), "LineOfText([Word(worl)])");
        assert_eq!(format!("{:?}", line4), "LineOfText([Word(d!)])");

        assert_eq!(line1.size, Size::new(29, 16));
        assert_eq!(line2.size, Size::new(17, 16));
        assert_eq!(line3.size, Size::new(31, 16));
        assert_eq!(line4.size, Size::new(13, 16));
    }

    #[test]
    fn test_line_of_text_that_cant_fit_a_character() {
        let attributed_string = AttributedString::new(String::from("Hello, world!"));
        let lines = attributed_string.lines();
        let line = lines.first().unwrap();

        let lines_of_text = LineOfText::from(&line, 10, 1.0);

        assert_eq!(lines_of_text.len(), 10);

        let line1 = &lines_of_text[0];
        let line2 = &lines_of_text[1];
        let line3 = &lines_of_text[2];
        let line4 = &lines_of_text[3];
        let line5 = &lines_of_text[4];
        let line6 = &lines_of_text[5];
        let line7 = &lines_of_text[6];
        let line8 = &lines_of_text[7];
        let line9 = &lines_of_text[8];
        let line10 = &lines_of_text[9];

        assert_eq!(format!("{:?}", line1), "LineOfText([Word(H)])");
        assert_eq!(format!("{:?}", line2), "LineOfText([Word(e)])");
        assert_eq!(format!("{:?}", line3), "LineOfText([Word(ll)])");
        assert_eq!(format!("{:?}", line4), "LineOfText([Word(o)])");
        assert_eq!(format!("{:?}", line5), "LineOfText([Word(, )])");
        assert_eq!(format!("{:?}", line6), "LineOfText([Word(w)])");
        assert_eq!(format!("{:?}", line7), "LineOfText([Word(o)])");
        assert_eq!(format!("{:?}", line8), "LineOfText([Word(rl)])");
        assert_eq!(format!("{:?}", line9), "LineOfText([Word(d)])");
        assert_eq!(format!("{:?}", line10), "LineOfText([Word(!)])");

        // As intended by this test, some of these characters are larger than
        // the maximum width given.
        assert_eq!(line1.size, Size::new(12, 16));
        assert_eq!(line2.size, Size::new(9, 16));
        assert_eq!(line3.size, Size::new(8, 16));
        assert_eq!(line4.size, Size::new(9, 16));
        assert_eq!(line5.size, Size::new(8, 16));
        assert_eq!(line6.size, Size::new(12, 16));
        assert_eq!(line7.size, Size::new(9, 16));
        assert_eq!(line8.size, Size::new(10, 16));
        assert_eq!(line9.size, Size::new(9, 16));
        assert_eq!(line10.size, Size::new(4, 16));
    }

    #[test]
    fn test_line_visual_size() {
        let text = "The quick brown fox jumps ";
        let text_without_space = "The quick brown fox jumps";

        let attributed_string1 = AttributedString::new(String::from(text));
        let attributed_lines = attributed_string1.lines();
        let line = attributed_lines.first().unwrap();
        let lines = LineOfText::from(line, 200, 1.0);
        let line_of_text = lines.first().unwrap();

        let attributed_string2 = AttributedString::new(String::from(text_without_space));
        let attributed_lines = attributed_string2.lines();
        let line = attributed_lines.first().unwrap();
        let lines = LineOfText::from(line, 200, 1.0);
        let line_of_text_without_space = lines.first().unwrap();

        assert_eq!(line_of_text.visual_size(), line_of_text_without_space.visual_size());
    }

    #[test]
    fn test_whole_text_single_line() {
        let attributed_string = AttributedString::new(String::from("Hello, world!"));

        let frame = Rectangle::new(0, 0, 100, 100);
        let text = WholeText::from(&attributed_string, frame, 1.0);

        assert_eq!(text.lines.len(), 1);
    }

    #[test]
    fn test_whole_text_lines_total_height() {
        let attributed_string = AttributedString::new(String::from("Hello, world!\nGoodbye, world!"));

        let frame = Rectangle::new(0, 0, 100, 100);
        let text = WholeText::from(&attributed_string, frame, 1.0);

        assert_eq!(text.lines_total_height(), 48);
    }

    #[test]
    fn test_whole_text_horizonal_alignment_left() {
        let attributed_string = AttributedString::new(String::from("Hello, world!\nGoodbye, world!"));

        let frame = Rectangle::new(50, 50, 100, 100);
        let mut text = WholeText::from(&attributed_string, frame, 1.0);
        text.align_horizontally(HorizontalAlignment::Left);

        let line1_position = &text.positions[0];
        let line2_position = &text.positions[1];

        assert_eq!(line1_position.x, 0);
        assert_eq!(line2_position.x, 0);
    }

    #[test]
    fn test_whole_text_horizonal_alignment_center() {
        let attributed_string = AttributedString::new(String::from("Hello, world!\nGoodbye, world!"));

        let frame = Rectangle::new(50, 50, 100, 100);
        let mut text = WholeText::from(&attributed_string, frame, 1.0);
        text.align_horizontally(HorizontalAlignment::Center);

        let line1_position = &text.positions[0];
        let line2_position = &text.positions[1];

        assert_eq!(line1_position.x, 5);
        assert_eq!(line2_position.x, 16);
    }

    #[test]
    fn test_whole_text_horizontal_alignment_center_with_trailing_space() {
        let x_with_space: i32;
        let x_without_space: i32;
        {
            let attributed_string = AttributedString::new(String::from("The quick brown fox jumps "));
            let frame = Rectangle::new(50, 50, 100, 100);
            let mut text = WholeText::from(&attributed_string, frame, 1.0);
            text.align_horizontally(HorizontalAlignment::Center);
            let line1_position = &text.positions[0];
            x_with_space = line1_position.x;
        }
        {
            let attributed_string = AttributedString::new(String::from("The quick brown fox jumps"));
            let frame = Rectangle::new(50, 50, 100, 100);
            let mut text = WholeText::from(&attributed_string, frame, 1.0);
            text.align_horizontally(HorizontalAlignment::Center);
            let line1_position = &text.positions[0];
            x_without_space = line1_position.x;
        }

        assert_eq!(x_with_space, x_without_space);
    }

    #[test]
    fn test_whole_text_horizonal_alignment_right() {
        let attributed_string = AttributedString::new(String::from("Hello, world!\nGoodbye, world!"));

        let frame = Rectangle::new(50, 50, 100, 100);
        let mut text = WholeText::from(&attributed_string, frame, 1.0);
        text.align_horizontally(HorizontalAlignment::Right);

        let line1_position = &text.positions[0];
        let line2_position = &text.positions[1];

        assert_eq!(line1_position.x, 10);
        assert_eq!(line2_position.x, 31);
    }

    #[test]
    fn test_whole_text_horizontal_alignment_right_with_trailing_space() {
        let x_with_space: i32;
        let x_without_space: i32;
        {
            let attributed_string = AttributedString::new(String::from("The quick brown fox jumps "));
            let frame = Rectangle::new(50, 50, 100, 100);
            let mut text = WholeText::from(&attributed_string, frame, 1.0);
            text.align_horizontally(HorizontalAlignment::Right);
            let line1_position = &text.positions[0];
            x_with_space = line1_position.x;
        }
        {
            let attributed_string = AttributedString::new(String::from("The quick brown fox jumps"));
            let frame = Rectangle::new(50, 50, 100, 100);
            let mut text = WholeText::from(&attributed_string, frame, 1.0);
            text.align_horizontally(HorizontalAlignment::Right);
            let line1_position = &text.positions[0];
            x_without_space = line1_position.x;
        }

        assert_eq!(x_with_space, x_without_space);
    }

    #[test]
    fn test_whole_text_vertical_alignment_top() {
        let attributed_string = AttributedString::new(String::from("Hello, world!\nGoodbye, world!"));

        let frame = Rectangle::new(50, 50, 100, 100);
        let mut text = WholeText::from(&attributed_string, frame, 1.0);
        text.align_vertically(VerticalAlignment::Top);

        let line1_position = &text.positions[0];
        let line2_position = &text.positions[1];

        assert_eq!(line1_position.y, 0);
        assert_eq!(line2_position.y, 16);
    }

    #[test]
    fn test_whole_text_vertical_alignment_middle() {
        let attributed_string = AttributedString::new(String::from("Hello, world!\nGoodbye, world!"));

        let frame = Rectangle::new(50, 50, 100, 100);
        let mut text = WholeText::from(&attributed_string, frame, 1.0);
        text.align_vertically(VerticalAlignment::Middle);

        let line1_position = &text.positions[0];
        let line2_position = &text.positions[1];

        assert_eq!(line1_position.y, 26);
        assert_eq!(line2_position.y, 42);
    }

    #[test]
    fn test_whole_text_vertical_alignment_bottom() {
        let attributed_string = AttributedString::new(String::from("Hello, world!\nGoodbye, world!"));

        let frame = Rectangle::new(50, 50, 100, 100);
        let mut text = WholeText::from(&attributed_string, frame, 1.0);
        text.align_vertically(VerticalAlignment::Bottom);

        let line1_position = &text.positions[0];
        let line2_position = &text.positions[1];

        assert_eq!(line1_position.y, 52);
        assert_eq!(line2_position.y, 68);
    }
}
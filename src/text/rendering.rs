use crate::graphics::Rectangle;
use crate::graphics::Point;
use crate::graphics::Size;
use crate::text::AttributedString;
use crate::text::AttributedSubstring;

/// Used for rendering.
///
/// A character to be rendered as part of `LineOfText`.
struct Character {
    /// The character to be rendered.
    character: char,

    /// We only care about the x position of the character, because the y
    /// position is determined by the line height (so is up to `LineOfText`).
    x: i32,

    /// The size of the character.
    size: Size<u32>
}

/// Used for rendering.
///
/// A whole line of text, containing characters of varying styles and sizes.
struct LineOfText {
    /// The text to be rendered.
    text: Vec<Character>,

    /// The position and size of the line.
    ///
    /// The height of the line is determined by the largest character in the
    /// line.
    ///
    /// The width of the line is determined by the sum of the widths of all
    /// characters in the line.
    frame: Rectangle<i32, u32>
}

struct WholeText {
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
    /// Calculate the x of the next character (the character following this
    /// one).
    fn next_character_x(&self) -> i32 {
        let width = self.size.width as i32;
        self.x + width
    }
}

impl LineOfText {
    /// Creates a `LineOfText` from an `AttributedSubstring`.
    ///
    /// The passed in `AttributedSubstring` is assumed to be a single line of
    /// text. For handling multiple lines, use `WholeText`.
    fn from(attributed_string: &AttributedSubstring, origin: Point<i32>) -> LineOfText {
        let mut characters: Vec<Character> = Vec::new();

        for (char_index, character) in attributed_string.chars().enumerate() {
            let font = attributed_string.font_for(char_index);
            let size = font.size_for(&String::from(character));

            let x = if let Some(previous_character) = characters.last() {
                previous_character.next_character_x()
            } else {
                0
            };

            let character_to_render = Character {
                character,
                x,
                size
            };

            characters.push(character_to_render);
        }

        // The height of the line is determined by the largest character in
        // the line.
        let height = characters.iter().map(|character| character.size.height).max().unwrap();

        // The width of the line is determined by the sum of the widths of
        // all characters in the line.
        let width = characters.iter().map(|character| character.size.width).sum();

        let size = Size::new(width, height);
        let frame = Rectangle { origin, size};

        LineOfText {
            text: characters,
            frame
        }
    }
}

impl WholeText {
    /// Creates a `rendering::WholeText` from an `AttributedString`.
    fn from(attributed_string: &AttributedString, origin: Point<i32>) -> WholeText {
        let mut lines: Vec<LineOfText> = Vec::new();

        for line in attributed_string.lines() {
            let y = if let Some(previous_line) = lines.last() {
                previous_line.frame.bottom()
            } else {
                0
            };

            let x = origin.x;

            let origin = Point::new(x, y);

            let line_of_text = LineOfText::from(&line, origin);

            lines.push(line_of_text);
        }

        let height = lines.iter().map(|line| line.frame.size.height).sum();
        let width = lines.iter().map(|line| line.frame.size.width).max().unwrap();

        let size = Size::new(width, height);
        let frame = Rectangle { origin, size};

        WholeText {
            text: lines,
            frame
        }
    }
}


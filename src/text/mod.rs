pub mod attributed_string;
pub mod rendering;
pub mod word_boundary;
pub mod text;

pub use text::Text;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom
}

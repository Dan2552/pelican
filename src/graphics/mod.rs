use crate::macros::singleton;

mod number;
use std::ops::Deref;

pub use number::Number;

mod context;
pub use context::Context;

mod point;
pub use point::Point;

mod size;
pub use size::Size;

mod rectangle;
pub use rectangle::Rectangle;

mod layer;
pub use layer::Layer;

pub use layer::LayerDelegate;

mod font;
pub use font::Font;

mod image;
pub use image::Image;

pub use sdl2::pixels::Color;

pub struct SdlContainer {
    sdl: sdl2::Sdl,
}

impl Default for SdlContainer {
    fn default() -> Self {
        let sdl = sdl2::init().unwrap();
        SdlContainer { sdl }
    }
}

impl Deref for SdlContainer {
    type Target = sdl2::Sdl;

    fn deref(&self) -> &Self::Target {
        &self.sdl
    }
}

singleton!(SdlContainer + Default);

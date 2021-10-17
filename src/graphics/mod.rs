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

// TODO: probably remove and reduce visibility to crate
pub use layer::LayerDelegate;

mod font;
pub use font::Font;

// mod image;
// pub use image::Image;

pub use sdl2::pixels::Color;

// TODO: might not be possible because e.g. Font needs the text size in order to make it. Thoguh maybe there's another object wrapping font for that purpose?
trait Drawable {
    fn layer_for(&self, context: Context) -> Layer;
}

// TODO: doesn't belong here. Belongs in platform
pub struct Bundle {}



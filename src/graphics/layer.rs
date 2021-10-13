use crate::graphics::Context;
use crate::graphics::Size;
use crate::graphics::Point;
use crate::graphics::Color;

use sdl2::render::Texture;
use sdl2::render::TextureAccess;

pub struct Layer<'a> {
    context: Context,
    texture: Texture<'a>,
    size: Size,
    needs_display: bool
}

impl<'a> Layer<'a> {
    pub fn new(context: Context, size: Size) -> Layer<'a> {
        let texture = context
            .texture_creator
            .create_texture(None, TextureAccess::Target, size.width, size.height)
            .unwrap();

        Layer {
            context: context,
            size: size,
            needs_display: true,
            texture: texture
        }
    }

    fn draw(&self) {
        // @needs_display = false
        // delegate.layer_will_draw(self)
        // delegate.draw_layer(self)
    }

    fn set_needs_display(mut self) {
        self.needs_display = true
    }

    fn draw_child_layer(&self, layer: Layer, position: Point, size: Size) {
        // c__draw_child_texture(layer, x, y, w, h)
    }

    fn clear_with_color(&self, color: Color) {
        // c__clear_with_color(red, green, blue, alpha)
        // self.context.canvas.target
    }
}

use crate::graphics::Context;
use crate::graphics::Point;
use crate::graphics::Size;
use crate::graphics::Color;
use crate::graphics::Rectangle;

use sdl2::render::Texture;
use sdl2::render::TextureAccess;
use sdl2::render::BlendMode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::rc::Rc;
use std::cell::RefCell;

// To use an analogy, this is a piece of paper that will be drawn on. It'll
// then either be glued onto another layer, or onto the `Context` canvas.
// These layers then make the full picture.
//
// It's important to note, the `Layer` itself doesn't really handle *what*
// is going to be drawn on it, or in what order. It instead delegates this
// behavior to `delegate`. Without a `delegate`, it wont draw anything.
pub struct Layer {
    context: Rc<Context>, // e.g. the window this layer is in
    texture: Rc<RefCell<Texture>>, // the texture this layer is drawn with

    // TODO: should the size be updated using the following somewhere? Maybe it cannot change though without layer changing it?
    // SDL_QueryTexture(texture, NULL, NULL, &width, &height);
    size: Size,

    needs_display: bool,
    delegate: Box<dyn LayerDelegate>
}

// TODO: probably pub(crate)
pub trait LayerDelegate {
    fn layer_will_draw(&self, layer: &Layer);
    fn draw_layer(&self, layer: &Layer);
}

impl Layer {
    // TODO: probably pub(crate)
    pub fn new(context: Rc<Context>, size: Size, delegate: Box<dyn LayerDelegate>) -> Layer {
        let mut texture = context.texture_creator
                .create_texture(None, TextureAccess::Target, size.width, size.height)
                .unwrap();

        texture.set_blend_mode(BlendMode::Blend);

        Layer {
            context: context,
            size: size,
            needs_display: true,
            texture: Rc::new(RefCell::new(texture)),
            delegate: delegate
        }
    }

    // TODO: pub(crate)
    pub fn draw(&mut self) {
        self.needs_display = false;
        self.delegate.layer_will_draw(&self);
        self.delegate.draw_layer(&self);
    }

    fn set_needs_display(mut self) {
        self.needs_display = true
    }

    // TODO: pub(crate)
    pub fn draw_child_layer(&self, child_layer: &Layer, destination: &Rectangle) {
        let mut parent_texture = self.texture.borrow_mut();
        let child_texture = child_layer.texture.borrow();
        let context = &self.context;

        context.draw_texture_in_texture(&mut parent_texture, &child_texture, &destination);
    }

    // Actually copies this layer's texture to the context canvas.
    pub fn draw_into_context(&self) {
        let context = &self.context;
        let texture = self.texture.borrow_mut();

        // TODO: clone size
        let rectangle = Rectangle {
            position: Point { x: 0, y: 0 },
            size: Size { width: self.size.width, height: self.size.height }
        };
        context.draw_texture_in_context(&texture, &rectangle);
    }

    pub fn clear_with_color(&self, color: Color) {
        let mut texture = self.texture.borrow_mut();
        let mut context = &self.context;

        context.clear_texture(&mut texture, color)
    }
}

use crate::graphics::Context;
use crate::graphics::Point;
use crate::graphics::Size;
use crate::graphics::Color;
use crate::graphics::Rectangle;

use sdl2::render::Texture;
use sdl2::render::TextureAccess;
use sdl2::render::BlendMode;

use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Cell;

/// To use an analogy, this is a piece of paper that will be drawn on. It'll
/// then either be glued onto another layer, or onto the `Context` canvas.
/// These layers then make the full picture.
///
/// It's important to note, the `Layer` itself doesn't really handle *what*
/// is going to be drawn on it, or in what order. It instead delegates this
/// behavior to `delegate`. Without a `delegate`, it wont draw anything.
pub struct Layer {
    context: Context, // e.g. the window this layer is in
    texture: Rc<RefCell<Texture>>, // the texture this layer is drawn with

    /// This is the portion of the underlying texture that is actually drawn.
    ///
    /// See `new_partial`.
    source_rectangle: Option<Rectangle<i32, u32>>,

    // TODO: should the size be updated using the following somewhere? Maybe it cannot change though without layer changing it?
    // SDL_QueryTexture(texture, NULL, NULL, &width, &height);
    size: Size<u32>,

    needs_display: Cell<bool>,

    /// This layer's scale.
    ///
    /// If the layer scale is 1.0 and the screen display is 2.0, when the layer
    /// is drawn, it will be scaled up by 2.0.
    ///
    /// If the layer scale is 2.0 and the screen display is 2.0, when the layer
    /// is drawn, it wont be scaled up at all.
    scale: f32,

    delegate: Box<dyn LayerDelegate>
}

// TODO: probably pub(crate)
pub trait LayerDelegate {
    fn layer_will_draw(&self, _layer: &Layer) {}
    fn draw_layer(&self, _layer: &Layer) {}

    /// Drawing layer has resized, but hopefully it's a case of shuffling things
    /// around rather than redrawing all children from scratch.
    ///
    fn layout_sublayers(&self, _layer: &Layer) {}
}

impl Layer {
    // TODO: probably pub(crate)
    pub fn new(context: Context, size: Size<u32>, delegate: Box<dyn LayerDelegate>) -> Layer {
        let width = size.width as f32 * context.render_scale();
        let height = size.height as f32 * context.render_scale();

        if width.round() as u32 != width as u32 {
            println!("Warning: Layer width is not an integer. This may cause rendering issues.");
        }

        if height.round() as u32 != height as u32 {
            println!("Warning: Layer height is not an integer. This may cause rendering issues.");
        }

        let mut texture = context.texture_creator()
            .create_texture(
                None,
                TextureAccess::Target,
                width.round() as u32,
                height.round() as u32
            ).unwrap();

        texture.set_blend_mode(BlendMode::Blend);

        Layer {
            context: context,
            size: size,
            needs_display: Cell::new(true),
            texture: Rc::new(RefCell::new(texture)),
            delegate: delegate,
            scale: 1.0,
            source_rectangle: None
        }
    }

    /// Create a layer with a texture already drawn. That is, a texture is
    /// passed in at construction, and there is no delegate to handle any draw
    /// instructions. Making `draw()` no-op.
    pub fn new_prerendered(context: Context, size: Size<u32>, texture: Texture, scale: f32) -> Self {
        Layer {
            context: context,
            size: size,
            needs_display: Cell::new(false),
            texture: Rc::new(RefCell::new(texture)),
            delegate: Box::new(EmptyLayerDelegate {}),
            scale: scale,
            source_rectangle: None
        }
    }

    /// Creates a layer that cannot draw anything on its own. It's useful for
    /// creating a layer that can be used as a container for other layers.
    pub fn new_no_render(context: Context, size: Size<u32>) -> Self {
        Layer::new(context, size, Box::new(EmptyLayerDelegate {}))
    }

    /// Creates a layer that can draw a partial portion of the underlying
    /// texture.
    ///
    /// The underlying texture itself is shared between all the layer from this
    /// instance and all the layers that are created from this instance.
    pub fn new_partial(&self, portion: Rectangle<i32, u32>) -> Self {
        let scaled_portion = &portion * self.scale;

        Layer {
            context: self.context.clone(),
            size: portion.size.clone(),
            needs_display: Cell::new(false),
            texture: self.texture.clone(),
            delegate: Box::new(EmptyLayerDelegate {}),
            scale: self.scale,
            source_rectangle: Some(scaled_portion)
        }
    }

    // Requests for the delegate to draw on this layer.
    pub fn draw(&self) {
        self.delegate.layer_will_draw(self);
        self.delegate.draw_layer(self);
        self.needs_display.set(false);
    }

    pub(crate) fn skip_draw(&self) {
        self.needs_display.set(false);
    }

    pub(crate) fn get_needs_display(&self) -> bool {
        self.needs_display.get()
    }

    pub(crate) fn set_needs_display(&self) {
        self.needs_display.set(true)
    }

    /// Set the color factor of the texture for the next render.
    pub fn set_color_factor(&self, color: Color, blend_factor: f32) {
        let mut texture = self.texture.borrow_mut();

        if blend_factor == 0.0 {
            texture.set_color_mod(255, 255, 255);
            return;
        }

        let red = color.r as f32;
        let green = color.g as f32;
        let blue = color.b as f32;

        let red = ((255.0 - red) * (1.0 - blend_factor)) + red;
        let green = ((255.0 - green) * (1.0 - blend_factor)) + green;
        let blue = ((255.0 - blue) * (1.0 - blend_factor)) + blue;

        let red = red.round() as u8;
        let green = green.round() as u8;
        let blue = blue.round() as u8;

        texture.set_color_mod(red, green, blue);
    }

    /// Note: The destination at this point is using the (unscaled) point
    /// system, not the real pixel size. The size of the real texture itself is
    /// determined by this method; the difference in quality being the source
    /// picture size.
    pub fn draw_child_layer(&self, child_layer: &Layer, destination: &Rectangle<i32, u32>) {
        let mut parent_texture = self.texture.borrow_mut();
        let child_texture = child_layer.texture.borrow();
        let context = &self.context;

        // Source is prescaled in `new_partial`.
        let source = child_layer.source_rectangle.as_ref();

        let destination = destination * context.render_scale();

        // An example at this point, after multiplying destination:
        // if the image was 1x, the src could be 55 and the dest 110
        // if the image was 2x, the equivalent src would be 110 and the dest
        // still 110
        context.draw_texture_in_texture(&mut parent_texture, &child_texture, source, &destination);
    }

    /// To be used when the layer is already declared at the native resolution.
    /// Used by rendering text (e.g. at twice the font size than specified)
    /// because each character is drawn separately to a layer first.
    pub fn draw_child_layer_without_scaling(&self, child_layer: &Layer, destination: &Rectangle<i32, u32>) {
        let mut parent_texture = self.texture.borrow_mut();
        let child_texture = child_layer.texture.borrow();
        let context = &self.context;

        context.draw_texture_in_texture(&mut parent_texture, &child_texture, None, &destination);
    }

    // Actually copies this layer's texture to the context canvas.
    pub fn draw_into_context(&self) {
        let context = &self.context;
        let texture = self.texture.borrow_mut();

        let rectangle = Rectangle {
            origin: Point { x: 0, y: 0 },
            size: self.size.clone()
        };

        let rectangle = &rectangle * self.context.render_scale();

        context.draw_texture_in_context(&texture, &rectangle);
    }

    pub fn clear_with_color(&self, color: Color) {
        let mut texture = self.texture.borrow_mut();
        let context = &self.context;

        context.clear_texture(&mut texture, color)
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Only to be used by tests
    pub fn _raw_texture(&self) -> std::cell::Ref<'_, Texture> {
        self.texture.borrow()
    }
}

impl PartialEq for Layer {
    fn eq(&self, rhs: &Layer) -> bool {
        std::ptr::eq(self, rhs)
    }
}

impl std::fmt::Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layer {{ size: {:?} }}", self.size)
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        // Get the raw pointer to the SDL_Texture
        let texture_ptr = self.texture.borrow().raw(); // or whatever method gives you the pointer

        unsafe {
            // Call SDL_DestroyTexture on the pointer
            sdl2::sys::SDL_DestroyTexture(texture_ptr);
        }
    }
}

struct EmptyLayerDelegate {}
impl LayerDelegate for EmptyLayerDelegate {}

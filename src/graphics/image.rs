use crate::graphics::Layer;
use crate::graphics::Size;
use crate::graphics::Context;
use crate::platform::Bundle;
use std::collections::HashMap;
use std::rc::Rc;
use sdl2::image::LoadSurface;

use sdl2::surface::Surface;

pub struct Image<'a> {
    size: Size<u32>,

    /// A texture (and therefore a layer created using one) is unique per
    /// context, so `Image` will lazily create `Layer` objects once per context.
    /// This are lazily populated by `layer_for()`.
    ///
    layers: HashMap<u32, Rc<Layer>>,

    surface: Surface<'a>
}

impl<'a> Image<'a> {
    pub fn new(name: &str) -> Image<'a> {
        let image_path = Bundle::path_for_resource(name);

        let surface = Surface::from_file(image_path).unwrap();

        let width = surface.width();
        let height = surface.height();
        let size = Size { width, height };
        let layers = HashMap::new();

        Image { size, layers, surface }
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    pub fn layer_for(&mut self, context: Rc<Context>) -> Rc<Layer> {
        let id = context.id;

        if self.layers.get(&id).is_none() {
            let texture = self.surface.as_texture(&context.texture_creator).unwrap();
            let layer = Layer::new_prerendered(context.clone(), self.size.clone(), texture);
            let layers = &mut self.layers;
            layers.insert(context.id, Rc::new(layer));
        }

        self.layers.get(&id).unwrap().clone()
    }
}


use crate::graphics::Layer;
use crate::graphics::Size;
use crate::graphics::Context;
use crate::platform::bundle::Bundle;
use std::collections::HashMap;
use std::rc::Rc;
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;
use regex::Regex;
use std::fs::metadata;

/// Represents images / pictures.
pub struct Image<'a> {
    name: String,

    size: Size<u32>,

    /// A texture (and therefore a layer created using one) is unique per
    /// context, so `Image` will lazily create `Layer` objects once per context.
    /// This are lazily populated by `layer_for()`.
    ///
    layers: HashMap<u32, Rc<Layer>>,

    surface: Surface<'a>,

    scale_loaded: u8
}

impl<'a> Image<'a> {
    /// Creates a new image.
    ///
    /// The `name` attribute is a path to the image file. If a relative path is
    /// given, it is relative to the application's `resource` directory.
    ///
    /// If an image with a @2x suffix (before the file extension) is found,
    /// it will be loaded and scaled appropriately if the display scale is
    /// greater than 1.
    pub fn new(name: &str) -> Image<'a> {
        let image_path_2x = Image::scale_2x_name(name);
        let image_path = Bundle::path_for_resource(name);

        let surface;
        let width;
        let height;
        let scale_loaded;

        // Regardless of what image gets picked here, if a more appropriate
        // one is found in `layer_for()`, as we will only know the target
        // display scale at that point, the image will be loaded again.
        if Image::is_file(&image_path_2x) {
            // By default, we load the 2x image if there is one. There's just
            // going to be a higher chance that modern displays are scaled.
            surface = Surface::from_file(image_path_2x).unwrap();
            width = (surface.width() as f32 * 0.5).round() as u32;
            height = (surface.height() as f32 * 0.5).round() as u32;
            scale_loaded = 2;
        } else if Image::is_file(&image_path) {
            // We load the regular image if there is no 2x image.
            surface = Surface::from_file(image_path).unwrap();
            width = surface.width();
            height = surface.height();
            scale_loaded = 1;
        } else {
            panic!("Image not found: {}. Searched the following paths: [\n  {},\n  {}\n]", name, image_path, image_path_2x);
        }

        let size = Size { width, height };
        let layers = HashMap::new();
        let name = name.to_string();

        Image { name, size, layers, surface, scale_loaded }
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    /// Returns the layer to be drawn for the given context.
    ///
    /// Note: Because the scale of the context may change, in the case the
    /// window is moved from one screen to another, this may reload the image
    /// from disk if a more appropriate scale version is found. This may also
    /// happen once because the scale wasn't known at initialization.
    pub fn layer_for(&mut self, context: &Context) -> Rc<Layer> {
        let id = context.id();
        let render_scale = context.render_scale();

        if render_scale == 1.0 && self.scale_loaded != 1 {
            let image_path = Bundle::path_for_resource(&self.name);

            if Image::is_file(&image_path) {
                self.scale_loaded = 1;
                self.surface = Surface::from_file(image_path).unwrap();
            }
        } else if render_scale == 2.0 && self.scale_loaded != 2 {
            let image_path_2x = Image::scale_2x_name(&self.name);

            if Image::is_file(&image_path_2x) {
                self.scale_loaded = 2;
                self.surface = Surface::from_file(image_path_2x).unwrap();
            }
        }

        if self.layers.get(&id).is_none() {
            let texture = self.surface.as_texture(context.texture_creator()).unwrap();
            let layer = Layer::new_prerendered(context.clone(), self.size.clone(), texture);
            let layers = &mut self.layers;
            layers.insert(id, Rc::new(layer));
        }

        self.layers.get(&id).unwrap().clone()
    }

    fn scale_2x_name(name: &str) -> String {
        let name2x = name.clone();
        let name2x = Regex::new(r"\.png$").unwrap().replace_all(&name2x, "@2x.png");
        let name2x = Regex::new(r"\.jpg$").unwrap().replace_all(&name2x, "@2x.jpg");
        let name2x = Regex::new(r"\.jpeg$").unwrap().replace_all(&name2x, "@2x.jpeg");
        Bundle::path_for_resource(&name2x)
    }

    fn is_file(path: &str) -> bool {
        match metadata(path) {
            Ok(metadata) => metadata.is_file(),
            Err(_) => false
        }
    }
}

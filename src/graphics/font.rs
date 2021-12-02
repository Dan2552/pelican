use crate::platform::Bundle;
use std::path::Path;
use crate::graphics::Context;
use crate::graphics::Layer;
use crate::graphics::Size;
use crate::graphics::Color;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::{RefCell, Ref};

pub struct Font<'ttf_module, 'rwops> {
    path: String,
    size: u16,

    /// A new font needs to be constructed for each desired font size. These are
    /// lazily created and cached so as to not need to repeatedly load the same
    /// fonts.
    font_sizes: RefCell<HashMap<u16, Rc<sdl2::ttf::Font<'ttf_module, 'rwops>>>>
}

const PATHS: &[&str] = &[
    "/System/Library/Fonts",
    "/System/Library/Fonts/Cache",
    "/System/Library/Fonts/Supplemental"
];

const TYPES: &[&str] = &[
    ".ttc",
    ".ttf",
    ".fon",
    ""
];

pub(crate) struct SdlTtfContainer {
    ttf: Option<Rc<sdl2::ttf::Sdl2TtfContext>>,
}

impl SdlTtfContainer {
    pub fn lazy(&mut self) -> &sdl2::ttf::Sdl2TtfContext {
        if self.ttf.is_some() {
            self.ttf.as_ref().unwrap()
        } else {
            self.ttf = Some(Rc::new(sdl2::ttf::init().unwrap()));
            self.ttf.as_ref().unwrap()
        }
    }
}

static mut TTF_CONTAINER: SdlTtfContainer = SdlTtfContainer {
    ttf: None
};

impl<'ttf_module, 'rwops> Font<'ttf_module, 'rwops> {
    pub fn new(font_name: &str, size: u16) -> Font<'ttf_module, 'rwops> {
        let bundle = Bundle::borrow();
        Font::new_with_bundle(font_name, size, &bundle)
    }

    pub fn new_with_bundle(font_name: &str, size: u16, bundle: &Bundle) -> Font<'ttf_module, 'rwops> {
        let path = find_font(font_name, bundle);
        let font_sizes = RefCell::new(HashMap::new());
        Font { path, size, font_sizes }
    }

    // Get a drawable layer from the font for the given context.
    pub fn layer_for(&mut self, context: Rc<Context>, text: &str, color: Color) -> Layer {
        let font_size = (self.size as f32 * context.render_scale) as u16;
        let font = self.load_font_for_size(font_size);
        let (width, height) = font.size_of(text).unwrap();

        let surface = font
            .render(text)
            .blended(color)
            .unwrap();

        let texture = surface.as_texture(&context.texture_creator).unwrap();

        Layer::new_prerendered(
            context.clone(),
            Size { width, height },
            texture
        )
    }

    /// Get the size of the given string for this font.
    pub fn size_of(&self, text: &str) -> Size<u32> {
        let font = self.load_font_for_size(self.size);
        let (width, height) = font.size_of(text).unwrap();
        Size { width, height }
    }

    /// Loads a font from the given size. This is a lazy operation, so the
    /// font will only be loaded if it is not already loaded (using 
    /// `self.font_sizes`).
    fn load_font_for_size(&self, font_size: u16) -> Rc<sdl2::ttf::Font<'ttf_module, 'rwops>> {
        let mut font_sizes = self.font_sizes.borrow_mut();

        if font_sizes.get(&font_size).is_none() {
            let ttf_context = unsafe { TTF_CONTAINER.lazy() };
            let font = ttf_context.load_font(&self.path, font_size).unwrap();
            font_sizes.insert(font_size, Rc::new(font));
        }

        font_sizes.get(&font_size).unwrap().clone()
    }
}

fn find_font(font_name: &str, bundle: &Bundle) -> String {
    // Find the font in system paths.
    for path in PATHS {
        for filetype in TYPES {
            let path = *path;
            let potential = format!("{}/{}{}", path, font_name, filetype);
            if Path::new(&potential).exists() {
                return potential;
            }
        }
    }

    // If it wasn't found in system paths, try the bundle path.
    for filetype in TYPES {
        let potential = bundle.path_for_resource(&format!("{}{}", font_name, filetype));
        if Path::new(&potential).exists() {
            return potential;
        }
    }

    // If we've still not found it, we're out of ideas.
    panic!("Font {} not found", font_name);
}

use crate::graphics::Size;
use crate::graphics::Point;
use crate::graphics::Rectangle;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;

// TODO: make all SdlContainer related pub(crate)

pub struct SdlContainer {
    sdl: Option<Rc<sdl2::Sdl>>,
}

impl SdlContainer {
    pub fn lazy(&mut self) -> &sdl2::Sdl {
        if self.sdl.is_some() {
            self.sdl.as_ref().unwrap()
        } else {
            self.sdl = Some(Rc::new(sdl2::init().unwrap()));
            self.sdl.as_ref().unwrap()
        }
    }
}

pub static mut SDL_CONTAINER: SdlContainer = SdlContainer {
    sdl: None
};

/// `Context` for a graphics render target. E.g. a window.
///
/// Each `Layer` for a given render target will use the `Context` to draw to
/// screen.
pub struct Context {
    pub id: uuid::Uuid,

    /// The size of the drawable canvas
    size: Size<u32>,

    /// The render scale. This would be different if using a higher density
    /// display.
    pub render_scale: f32,

    /// Internal SDL canvas
    canvas: Rc<RefCell<Canvas<Window>>>,

    /// Internal SDL texture creator
    pub(crate) texture_creator: TextureCreator<WindowContext>,

    // TODO: which one of these is the same as `size: Size`?
    render_size: Size<u32>,
    pixel_size: Size<u32>
}

impl Context {
    pub fn new(title: &str, position: Point<i32>, size: Size<u32>) -> Context {
        let sdl: &sdl2::Sdl;
        unsafe { sdl = SDL_CONTAINER.lazy(); }

        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window(title, size.width, size.height)
            .position(position.x, position.y)
            .opengl()
            .allow_highdpi()
            .build()
            .unwrap();

        let (render_width, render_height) = window.size();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let (pixel_width, pixel_height) = canvas.output_size().unwrap();

        let render_size = Size { width: render_width, height: render_height };
        let pixel_size = Size { width: pixel_width, height: pixel_height  };

        let render_scale = pixel_width as f32 / size.width as f32;

        let texture_creator = canvas.texture_creator();

        Context {
            id: uuid::Uuid::new_v4(),
            size: size,
            render_scale: render_scale,
            canvas: Rc::new(RefCell::new(canvas)),
            render_size: render_size,
            pixel_size: pixel_size,
            texture_creator: texture_creator
        }
    }

    // TODO: pub(crate)
    pub fn draw(&self) {
        println!("present");
        let mut canvas = self.canvas.borrow_mut();
        canvas.present();
    }

    // TODO: pub(crate)
    pub fn draw_texture_in_context(&self, child: &Texture, destination: &Rectangle<i32, u32>) {
        let destination = Rect::new(
            destination.position.x,
            destination.position.y,
            destination.size.width,
            destination.size.height
        );

        let mut canvas = self.canvas.borrow_mut();
        println!("drawing texture into canvas");
        canvas.copy(child, None, destination).unwrap();
    }

    pub(crate) fn draw_texture_in_texture(&self, parent: &mut Texture, child: &Texture, destination: &Rectangle<i32, u32>) {
        let destination = Rect::new(
            destination.position.x.try_into().unwrap(),
            destination.position.y.try_into().unwrap(),
            destination.size.width,
            destination.size.height
        );

        let mut canvas = self.canvas.borrow_mut();

        canvas.with_texture_canvas(parent, |canvas| {
            println!("drawing texture into another texture");
            canvas.copy(&child, None, destination).unwrap();
        }).unwrap();
    }

    pub(crate) fn clear_texture(&self, texture: &mut Texture, color: Color) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.with_texture_canvas(texture, |canvas| {
            println!("clearing texture with color");
            canvas.set_draw_color(color);
            canvas.clear();
        }).unwrap();
    }
}

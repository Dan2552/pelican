use crate::graphics::Size;
use crate::graphics::Point;
use crate::graphics::Layer;
use crate::graphics::Rectangle;

use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;

// Context for a graphics render target. E.g. a window.
pub struct Context {
    size: Size,
    pub render_scale: f32,
    canvas: Rc<RefCell<Canvas<Window>>>,
    pub(crate) texture_creator: TextureCreator<WindowContext>,
    // TODO: which one of these is the same as `size: Size`?
    render_size: Size,
    pixel_size: Size
}

impl Context {
    pub fn new(sdl: &sdl2::Sdl, title: &str, position: Point, size: Size) -> Context {
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window(title, size.width, size.height)
            .position(position.x.try_into().unwrap(), position.y.try_into().unwrap())
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
        let pixel_size = Size { width: pixel_width, height: pixel_height };

        let render_scale = pixel_width as f32 / size.width as f32;

        let texture_creator = canvas.texture_creator();

        Context {
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
        let mut canvas = self.canvas.borrow_mut();
        canvas.clear();
        canvas.present();
    }

    // TODO: pub(crate)
    pub fn draw_texture_in_context(&self, child: &Texture, destination: &Rectangle) {
        let destination = Rect::new(
            destination.position.x.try_into().unwrap(),
            destination.position.y.try_into().unwrap(),
            destination.size.width,
            destination.size.height
        );

        let mut canvas = self.canvas.borrow_mut();
        canvas.copy(child, None, destination).unwrap();
    }

    pub(crate) fn draw_texture_in_texture(&self, parent: &mut Texture, child: &Texture, destination: &Rectangle) {
        let destination = Rect::new(
            destination.position.x.try_into().unwrap(),
            destination.position.y.try_into().unwrap(),
            destination.size.width,
            destination.size.height
        );

        let mut canvas = self.canvas.borrow_mut();

        canvas.with_texture_canvas(parent, |canvas| {
            canvas.copy(&child, None, destination).unwrap();
        }).unwrap();
    }

    pub(crate) fn clear_texture(&self, texture: &mut Texture, color: Color) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.with_texture_canvas(texture, |canvas| {
            canvas.set_draw_color(color);
        }).unwrap();
    }

    // pub(crate) fn inside_texture<F>(self, texture: Texture, f: F) where for<'r> F: FnOnce(&'r mut Canvas<sdl2::video::Window>),{
    //     let mut canvas = self.canvas;
    //     let mut texture = texture;
    //     canvas.with_texture_canvas(&mut texture, |texture_canvas| {
    //         f(texture_canvas);
    //     }).unwrap();
    // }
}

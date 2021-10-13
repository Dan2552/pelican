use crate::graphics::Size;
use crate::graphics::Point;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use std::convert::TryInto;

pub struct Context {
    size: Size,
    pub render_scale: f32,
    pub(crate) canvas: Canvas<Window>,
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
        canvas.set_draw_color(Color::RGB(0, 255, 0));
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
            canvas: canvas,
            render_size: render_size,
            pixel_size: pixel_size,
            texture_creator: texture_creator
        }
    }

    fn draw(&self) {
        // TODO: c__orbit
    }
}

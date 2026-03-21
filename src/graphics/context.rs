use crate::graphics::SdlContainer;
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

/// `Context` for a graphics render target. E.g. a window.
///
/// Each `Layer` for a given render target will use the `Context` to draw to
/// screen.
pub struct Context {
    inner: Rc<ContextInner>
}

struct ContextInner {
    id: u32,

    /// The point size of the drawable canvas.
    ///
    /// As opposed to the actual pixel size: `pixel_size`. This would be
    /// different to `pixel_size` if the display has has higher DPI.
    size: Size<u32>,

    /// The actual pixel size of the drawable canvas.
    ///
    /// As opposed to the point size: `size`.
    pixel_size: Size<u32>,

    /// The render scale. This would be different if using a higher density
    /// display.
    render_scale: f32,

    /// Internal SDL canvas
    canvas: Rc<RefCell<Canvas<Window>>>,

    /// Internal SDL texture creator
    texture_creator: TextureCreator<WindowContext>
}

impl Context {
    pub fn new(title: &str, position: Point<i32>, size: Size<u32>) -> Context {
        let sdl = SdlContainer::borrow();
        let video_subsystem = sdl.video().expect("failed to initialize SDL video subsystem");

        let window = video_subsystem
            .window(title, size.width, size.height)
            .position(position.x, position.y)
            .opengl()
            .allow_highdpi()
            .build()
            .expect("failed to create SDL window");

        let id = window.id();
        let (render_width, render_height) = window.size();

        let mut canvas = window.into_canvas().build().expect("failed to create SDL canvas");

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let (pixel_width, pixel_height) = canvas.output_size().expect("failed to get canvas output size");

        let render_size = Size { width: render_width, height: render_height };
        let pixel_size = Size { width: pixel_width, height: pixel_height  };

        assert_eq!(render_size, size);

        let render_scale = pixel_width as f32 / size.width as f32;

        let texture_creator = canvas.texture_creator();

        Context {
            inner: Rc::new(ContextInner {
                id: id,
                size: size,
                render_scale: render_scale,
                canvas: Rc::new(RefCell::new(canvas)),
                pixel_size: pixel_size,
                texture_creator: texture_creator
            })
        }
    }

    pub fn id(&self) -> u32 {
        self.inner.id
    }

    pub(crate) fn texture_creator(&self) -> &TextureCreator<WindowContext> {
        &self.inner.texture_creator
    }

    pub fn render_scale(&self) -> f32 {
        self.inner.render_scale
    }

    pub fn size(&self) -> Size<u32> {
        self.inner.size.clone()
    }

    pub fn pixel_size(&self) -> Size<u32> {
        self.inner.pixel_size.clone()
    }

    pub fn draw(&self) {
        let mut canvas = self.inner.canvas.borrow_mut();
        canvas.present();
    }

    pub(crate) fn draw_texture_in_context(&self, child: &Texture, destination: &Rectangle<i32, u32>) {
        let destination = Rect::new(
            destination.origin.x,
            destination.origin.y,
            destination.size.width,
            destination.size.height
        );

        let mut canvas = self.inner.canvas.borrow_mut();
        canvas.copy(child, None, destination).expect("failed to copy texture to canvas");
    }

    pub(crate) fn draw_texture_in_texture(&self, parent: &mut Texture, child: &Texture, source: Option<&Rectangle<i32, u32>>, destination: &Rectangle<i32, u32>) {
        let source_rect;
        if let Some(source) = source {
            source_rect = Some(Rect::new(
                source.origin.x.try_into().expect("source x out of i32 range"),
                source.origin.y.try_into().expect("source y out of i32 range"),
                source.size.width,
                source.size.height
            ));
        } else {
            source_rect = None;
        }

        let destination = Rect::new(
            destination.origin.x.try_into().expect("destination x out of i32 range"),
            destination.origin.y.try_into().expect("destination y out of i32 range"),
            destination.size.width,
            destination.size.height
        );

        let mut canvas = self.inner.canvas.borrow_mut();

        canvas.with_texture_canvas(parent, |canvas| {
            canvas.copy(&child, source_rect, destination).expect("failed to copy texture");
        }).expect("failed to render to texture");
    }

    pub(crate) fn clear_texture(&self, texture: &mut Texture, color: Color) {
        let mut canvas = self.inner.canvas.borrow_mut();

        canvas.with_texture_canvas(texture, |canvas| {
            canvas.set_draw_color(color);
            canvas.clear();
        }).expect("failed to clear texture");
    }

    #[cfg(target_os = "macos")]
    pub fn ns_content_view(&self) -> Option<std::ptr::NonNull<std::ffi::c_void>> {
        use sdl2::raw_window_handle::{SDL_SysWMinfo, SDL_bool};
        use objc::runtime::Object;
        use objc::{msg_send, sel, sel_impl};

        extern "C" {
            fn SDL_GetWindowWMInfo(
                window: *mut sdl2::sys::SDL_Window,
                info: *mut SDL_SysWMinfo,
            ) -> SDL_bool;
        }

        let canvas = self.inner.canvas.borrow();
        let raw_window = canvas.window().raw();

        let mut wm_info: SDL_SysWMinfo = unsafe { std::mem::zeroed() };
        unsafe {
            sdl2::sys::SDL_GetVersion(&mut wm_info.version);
            if SDL_GetWindowWMInfo(raw_window, &mut wm_info) == SDL_bool::SDL_FALSE {
                return None;
            }
        }

        let ns_window = unsafe { wm_info.info.cocoa }.window;
        if ns_window.is_null() {
            return None;
        }

        let ns_view: *mut Object = unsafe { msg_send![ns_window as *mut Object, contentView] };
        std::ptr::NonNull::new(ns_view as *mut std::ffi::c_void)
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            inner: self.inner.clone()
        }
    }
}

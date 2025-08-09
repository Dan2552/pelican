use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use wry::raw_window_handle::{AppKitWindowHandle, HandleError, HasWindowHandle, RawWindowHandle, WindowHandle};
use std::ptr::NonNull;
use std::time::Duration;
use wry::WebViewBuilder;
use std::ffi::c_void;

pub struct NsContentViewParent {
    ns_view: NonNull<c_void>, // NSView* (NSWindow.contentView)
}

impl HasWindowHandle for NsContentViewParent {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        let appkit = AppKitWindowHandle::new(self.ns_view); // takes NSView*
        let raw = RawWindowHandle::AppKit(appkit);
        // SAFETY: we're creating a borrowed handle from a stable pointer
        Ok(unsafe { WindowHandle::borrow_raw(raw) })
    }
}

#[cfg(target_os = "macos")]
unsafe fn ns_window_from_metal(window: &sdl2::video::Window) -> *mut c_void {
    use sdl2::sys::{SDL_Metal_CreateView, SDL_Metal_DestroyView};
    use objc::{msg_send, sel, sel_impl};
    use objc::runtime::Object;

    let mview = SDL_Metal_CreateView(window.raw());
    if mview.is_null() {
        return std::ptr::null_mut();
    }

    let nsview: *mut Object = mview.cast();
    let nswindow: *mut Object = msg_send![nsview, window];

    SDL_Metal_DestroyView(mview);
    nswindow.cast() // NSWindow*
}

#[cfg(target_os = "macos")]
unsafe fn ns_content_view_from_metal(window: &sdl2::video::Window) -> Option<NonNull<c_void>> {
    use objc::{msg_send, sel, sel_impl};
    use objc::runtime::Object;

    let nswindow = ns_window_from_metal(window) as *mut Object;
    if nswindow.is_null() {
        return None;
    }
    let content_view: *mut Object = msg_send![nswindow, contentView]; // NSView*
    NonNull::new(content_view.cast())
}

fn main() -> wry::Result<()> {
    let sdl = sdl2::init().expect("Failed to init SDL");
    let video = sdl.video().expect("Failed to init SDL video");

    let window = video
        .window("SDL + WRY", 900, 600)
        .position_centered()
        .resizable()
        .metal_view()
        .build()
        .expect("Failed to create SDL window");

    unsafe { sdl2::sys::SDL_StopTextInput(); }

    let builder = WebViewBuilder::new()
        .with_transparent(true)
        .with_html(
          r#"<html>
                <body style="margin:0;background:rgba(87,87,87,0.5);display:grid;place-items:center;height:100vh;">
                    <div>hello webview <input autofocus type="text" placeholder="Type here..."></div>
                </body>
                </html>"#,
        )
        .with_devtools(true);

    let parent = NsContentViewParent { ns_view: unsafe { ns_content_view_from_metal(&window).unwrap() } };
    let webview = builder.build_as_child(&parent)?;

    webview.focus()?;

    let _ = webview.set_bounds(wry::Rect {
        position: dpi::Position::Logical(dpi::LogicalPosition { x: 0.0, y: 0.0 }),
        size: dpi::Size::Logical(dpi::LogicalSize { width: 300 as f64, height: 300 as f64 }),
    });

    // Event loop
    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(8));
    }

    Ok(())
}

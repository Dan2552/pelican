use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;
use std::time::Duration;
use wry::{WebView, WebViewBuilder};


fn main() -> wry::Result<()> {
    // SDL setup
    let sdl = sdl2::init().expect("Failed to init SDL");
    let video = sdl.video().expect("Failed to init SDL video");

    let window = video
        .window("SDL + WRY", 900, 600)
        .position_centered()
        .resizable()
        .allow_highdpi()
        .metal_view()
        .build()
        .expect("Failed to create SDL window");

    // Simple SDL render stuff just so you can see the background “beneath” the webview
    let mut canvas = window.clone()
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to get canvas");

    // Create the webview as a CHILD of the SDL window
    // `sdl2::video::Window` implements `HasWindowHandle`, which WRY accepts.
    let webview = attach_webview_to_sdl_window(&window)?;
    let (render_width, render_height) = window.size();


    let (pixel_width, _pixel_height) = canvas.output_size().unwrap();
    let render_scale = pixel_width as f64 / render_width as f64;


    // Consider scale
    let w = (render_width as f64 * render_scale).round() as i32;
    let h = (render_height as f64 * render_scale).round() as i32;

    // Keep the child webview sized to the SDL window
    let _ = webview.set_bounds(wry::Rect {
        position: dpi::Position::Physical(
            dpi::PhysicalPosition {
                x: 0,
                y: 0,
            }
        ),
        size: dpi::Size::Physical(
            dpi::PhysicalSize {
                width: w.max(1) as u32,
                height: h.max(1) as u32,
            }
        ),
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
                Event::Window { win_event, .. } => {
                    use sdl2::event::WindowEvent;
                    match win_event {
                        WindowEvent::Resized(w, h) | WindowEvent::SizeChanged(w, h) => {
                            // Consider scale
                            let w = (w as f64 * render_scale).round() as i32;
                            let h = (h as f64 * render_scale).round() as i32;

                            // Keep the child webview sized to the SDL window
                            let _ = webview.set_bounds(wry::Rect {
                                position: dpi::Position::Physical(
                                    dpi::PhysicalPosition {
                                        x: 0,
                                        y: 0,
                                    }
                                ),
                                size: dpi::Size::Physical(
                                    dpi::PhysicalSize {
                                        width: w.max(1) as u32,
                                        height: h.max(1) as u32,
                                    }
                                ),
                            });
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Draw a background (webview is a native child surface on top)
        canvas.set_draw_color(Color::RGB(32, 36, 48));
        canvas.clear();
        canvas.present();

        // Sleep a tad so we’re not pegging a core
        std::thread::sleep(Duration::from_millis(8));
    }

    Ok(())
}

fn attach_webview_to_sdl_window(win: &Window) -> wry::Result<WebView> {
    let builder = WebViewBuilder::new()
        .with_url("https://search.brave.com/")
        .with_devtools(true);

    let webview = builder.build_as_child(win)?;

    Ok(webview)
}

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use wry::WebViewBuilder;

fn main() -> wry::Result<()> {
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

    let mut canvas = window.clone()
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to get canvas");

    let builder = WebViewBuilder::new()
        .with_transparent(true)
        .with_html(
          r#"<html>
                <body style="background-color:rgba(87,87,87,0.5);">
                hello webview
                <input type="text" placeholder="Type here..." style="width: 200px; padding: 10px; font-size: 16px;"/>
                </body>
            </html>"#,
        )
        .with_devtools(true);

    let webview = builder.build_as_child(&window)?;
    webview.focus()?;

    let (w, h) = window.size();
    let _ = webview.set_bounds(wry::Rect {
        position: dpi::Position::Logical(dpi::LogicalPosition { x: 0.0, y: 0.0 }),
        size: dpi::Size::Logical(dpi::LogicalSize { width: w as f64, height: h as f64 }),
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
                            let _ = webview.set_bounds(wry::Rect {
                                position: dpi::Position::Logical(dpi::LogicalPosition { x: 0.0, y: 0.0 }),
                                size: dpi::Size::Logical(dpi::LogicalSize { width: w as f64, height: h as f64 }),
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
        webview.focus()?;

        // Sleep a tad so we’re not pegging a core
        std::thread::sleep(Duration::from_millis(8));
    }

    Ok(())
}

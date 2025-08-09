use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use wry::WebViewBuilder;

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

    let webview = builder.build_as_child(&window)?;

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

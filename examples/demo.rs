extern crate sdl2;

use pelican::graphics::Point;
use pelican::graphics::Size;
use pelican::graphics::Context;
use pelican::graphics::Layer;
use pelican::graphics::Rectangle;
use pelican::graphics::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;

struct FakeView {
    a: u32
}

impl pelican::graphics::LayerDelegate for FakeView  {
    fn layer_will_draw(&self, layer: &Layer) {

    }

    fn draw_layer(&self, layer: &Layer) {
        if self.a == 1 {
            let color = Color::RGBA(255, 0, 0, 255);
            layer.clear_with_color(color);
        } else {
            let color = Color::RGBA(0, 0, 255, 255);
            layer.clear_with_color(color);
        }
    }
}


pub fn main() -> Result<(), String> {
    let sdl = sdl2::init().unwrap();

    let position = Point { x: 10, y: 10};
    let size = Size { width: 800, height: 600 };
    let window1 = Rc::new(Context::new(&sdl, "hello world", position, size));

    // let context_reference1 = Rc::new(RefCell::new(window1));
    // let context_reference2 = context_reference1.clone();
    let size = Size { width: 800, height: 600 };
    let delegate = FakeView { a: 1 };
    let mut layer1 = Layer::new(window1.clone(), size, Box::new(delegate));

    let size = Size { width: 50, height: 50 };
    let delegate = FakeView { a: 2 };
    let mut layer2 = Layer::new(window1.clone(), size, Box::new(delegate));

    let rectangle = Rectangle {
        position: Point { x: 10, y: 10 },
        size: Size { width: 50, height: 50 }
    };

    layer1.draw();
    layer2.draw();
    layer1.draw_child_layer(&layer2, &rectangle);

    window1.draw();



    // let position = Point { x: 100, y: 100};
    // let size = Size { width: 300, height: 300 };
    // let window2 = Context::new(&sdl, "hello world", position, size);

    let mut event_pump = sdl.event_pump()?;

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

    //     canvas.clear();
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    //     // The rest of the game loop goes here...
    }

    Ok(())
}

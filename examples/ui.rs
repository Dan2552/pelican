use pelican::graphics::{Rectangle, Point, Size};
use pelican::ui::Window;
use pelican::ui::{ApplicationMain, ApplicationDelegate};

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {


        let frame = Rectangle {
            position: Point { x: 10, y: 10 },
            size: Size { width: 300, height: 300 }
        };
        let window = Window::new("hello world", frame);
    }
}

pub fn main() -> Result<(), String> {
    let app_delegate = AppDelegate {};
    let application_main = ApplicationMain::new(Box::new(app_delegate));
    application_main.launch();
    Ok(())
}

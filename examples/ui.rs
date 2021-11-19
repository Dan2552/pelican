use pelican::graphics::{Rectangle, Point, Size};
use pelican::ui::{View, Window};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        println!("view_did_load");
    }

    fn view_will_appear(&self, view: View) {
        println!("view_will_appear");
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle {
            position: Point { x: 10, y: 10 },
            size: Size { width: 300, height: 300 }
        };

        // TODO: is it possible to make it so you just pass in
        // ExampleViewController and it gets automatically coerced to the
        // wrapped ViewController?
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("hello world", frame, view_controller);

    }
}

pub fn main() -> Result<(), String> {
    let app_delegate = AppDelegate {};
    let application_main = ApplicationMain::new(Box::new(app_delegate));
    application_main.launch();
    Ok(())
}

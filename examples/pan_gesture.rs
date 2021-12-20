use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::gesture::pan_recognizer::PanRecognizer;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        let frame = Rectangle::new(0, 0, 10, 10);
        let red = View::new(frame);
        red.set_background_color(Color::red());
        view.add_subview(red);

        let pan_gesture = PanRecognizer::new(|_reconizer| {
            println!("Pan gesture recognized");
        });
        view.add_gesture_recognizer(Box::new(pan_gesture));
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(200, 200, 400, 200);
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("Pan example", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let app_delegate = AppDelegate {};
    let application_main = ApplicationMain::new(Box::new(app_delegate));
    application_main.launch();
    Ok(())
}

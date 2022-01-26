use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::button::Button;
use pelican::ui::TextField;

static mut n: usize = 0;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        let text_field = TextField::new(Rectangle::new(10, 10, 180, 32), "Hello, world!".to_string());
        let text_field_clone = text_field.clone();
        text_field.view.set_background_color(Color::gray());
        view.add_subview(text_field.view);

        let frame = Rectangle::new(10, 50, 180, 32);
        let button = Button::new(frame, "Carat", move || {
            // unsafe { text_field_clone.spawn_carat(n) };
            // println!("spawned carat");
            // unsafe { n = n + 1 };
            text_field_clone.select(0, 0, 5);
        });
        view.add_subview(button.view);
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(200, 200, 200, 200);
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("Text field example", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let app_delegate = AppDelegate {};
    let application_main = ApplicationMain::new(Box::new(app_delegate));
    application_main.launch();
    Ok(())
}

use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::TextField;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        let background = View::new(Rectangle::new(0, 0, view.frame().width(), view.frame().height()));
        background.set_background_color(Color::gray());
        view.add_subview(background.clone());

        let text_field = TextField::new(Rectangle::new(10, 10, 180, 36 * 2), "".to_string());
        text_field.view.set_background_color(Color::white());
        background.add_subview(text_field.clone().view);

        text_field.on_text_change(|text_field| {
            println!("text changed: {}", text_field.label().text());
        });

        text_field.view.become_first_responder();
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
    let application_main = ApplicationMain::new(AppDelegate {});
    application_main.launch();
    Ok(())
}

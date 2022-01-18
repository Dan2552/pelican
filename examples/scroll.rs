use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::ScrollView;
use pelican::ui::button::Button;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        // Setup the scrollview the same size as the window.
        let frame = Rectangle::new(0, 0, 400, 200);
        let scroll_view = ScrollView::new(frame);

        // View as "content" for the scrollview.
        let content_view = View::new(Rectangle::new(0, 0, 800, 400));

        let frame = Rectangle::new(0, 50, 400, 400 - 100);
        let child = View::new(frame);
        child.set_background_color(Color::gray());
        content_view.add_subview(child);

        let frame = Rectangle::new(0, 100, 100, 30);
        let button = Button::new(frame, "Button", move || {
            println!("button tapped");
        });
        button.view.set_background_color(Color::white());
        content_view.add_subview(button.view);

        scroll_view.set_content_view(content_view);
        view.add_subview(scroll_view.view);
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(200, 200, 400, 200);
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("Scroll example", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let app_delegate = AppDelegate {};
    let application_main = ApplicationMain::new(Box::new(app_delegate));
    application_main.launch();
    Ok(())
}

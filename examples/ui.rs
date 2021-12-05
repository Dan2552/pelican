use pelican::graphics::{Point, Rectangle, Image};
use pelican::ui::{View, Window, Color, ImageView, Label, Button};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::{HorizontalAlignment, VerticalAlignment};

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        let image = Image::new("/Users/dan2552/Dropbox/experiments/avian/pelican/test_application/resources/pixels_ruler.png");
        let image_view = ImageView::new(image, Point { x: 0, y: 0 });
        view.add_subview(image_view.view);

        let frame = Rectangle::new(0, 0, 200, 200);
        let label = Label::new(frame, String::from("hello rusty world\nhello hello"));
        label.set_text_alignment(HorizontalAlignment::Center);
        label.set_vertical_alignment(VerticalAlignment::Center);
        label.view.set_background_color(Color::red());
        // label.fit_to_text();

        view.add_subview(label.view);

        let frame = Rectangle::new(0, 40, 50, 50);
        let red_view = View::new(frame);
        red_view.set_background_color(Color::new(255, 0, 0, 100));

        let frame = Rectangle::new(50, 50, 50, 50);
        let green_view = View::new(frame);
        green_view.set_background_color(Color::new(0, 255, 0, 100));

        let frame = Rectangle::new(100, 60, 50, 50);
        let blue_view = View::new(frame);
        blue_view.set_background_color(Color::new(0, 0, 255, 100));

        let frame = Rectangle::new(200, 200, 200, 200);
        let button = Button::new(frame, "this is a button");

        view.add_subview(red_view);
        view.add_subview(green_view);
        view.add_subview(blue_view);
        view.add_subview(button.view);
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(100, 100, 500, 500);

        // TODO: is it possible to make it so you just pass in
        // ExampleViewController and it gets automatically coerced to the
        // wrapped ViewController?
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("hello world", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let app_delegate = AppDelegate {};
    let application_main = ApplicationMain::new(Box::new(app_delegate));
    application_main.launch();
    Ok(())
}

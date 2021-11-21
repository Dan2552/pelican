use pelican::graphics::{Point, Rectangle, Image};
use pelican::ui::{View, Window, Color, ImageView};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::platform::Bundle;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        println!("view_did_load");

        let bundle = Bundle::borrow();
        let image = Image::new("/Users/dan2552/Dropbox/experiments/avian/pelican/test_application/resources/pixels_ruler.png", &bundle);
        let image_view = ImageView::new(image, Point { x: 0, y: 0 });
        view.add_subview(image_view.view);

        let frame = Rectangle::new(10, 10, 30, 30);
        let child_view = View::new(frame);
        child_view.set_background_color(Color::red());
        view.add_subview(child_view);
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(10, 10, 480, 320);

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

use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color, Label, Button};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::text::{HorizontalAlignment, VerticalAlignment};
use pelican::text::attributed_string::AttributedString;
use pelican::text::attributed_string::Key;
use pelican::text::attributed_string::Attribute;

static PADDING: i32 = 10;
static INNER_WIDTH: u32 = 200;
static BUTTON_HEIGHT: u32 = 28;
static BUTTON_START: i32 = 230;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        let frame = Rectangle::new(PADDING, PADDING, INNER_WIDTH, INNER_WIDTH);
        let text = String::from("The quick brown fox jumps over the lazy dog");
        let label = Label::new(frame, text.clone());
        label.set_text_alignment(HorizontalAlignment::Center);
        label.set_vertical_alignment(VerticalAlignment::Middle);
        label.view.set_background_color(Color::gray());
        view.add_subview(label.view.clone());

        let attributed_string = AttributedString::new(text);
        let red = Color::red().to_graphics_color();

        // highlight the word "fox"
        attributed_string.set_attribute_for(16, Key::Color, Attribute::Color { color: red });
        attributed_string.set_attribute_for(17, Key::Color, Attribute::Color { color: red });
        attributed_string.set_attribute_for(18, Key::Color, Attribute::Color { color: red });

        label.set_attributed_text(attributed_string);

        let label_clone = label.clone();
        let frame = Rectangle::new(PADDING, BUTTON_START + ((PADDING + BUTTON_HEIGHT as i32) * 0), INNER_WIDTH, BUTTON_HEIGHT);
        let button = Button::new(frame, "Center", move || {
            label_clone.set_text_alignment(HorizontalAlignment::Center);
        });
        view.add_subview(button.view);

        let label_clone = label.clone();
        let frame = Rectangle::new(PADDING, BUTTON_START + ((PADDING + BUTTON_HEIGHT as i32) * 1), INNER_WIDTH, BUTTON_HEIGHT);
        let button = Button::new(frame, "Left", move || {
            label_clone.set_text_alignment(HorizontalAlignment::Left);
        });
        view.add_subview(button.view);

        let label_clone = label.clone();
        let frame = Rectangle::new(PADDING, BUTTON_START + ((PADDING + BUTTON_HEIGHT as i32) * 2), INNER_WIDTH, BUTTON_HEIGHT);
        let button = Button::new(frame, "Right", move || {
            label_clone.set_text_alignment(HorizontalAlignment::Right);
        });
        view.add_subview(button.view);

        let label_clone = label.clone();
        let frame = Rectangle::new(PADDING, BUTTON_START + ((PADDING + BUTTON_HEIGHT as i32) * 3), INNER_WIDTH, BUTTON_HEIGHT);
        let button = Button::new(frame, "Top", move || {
            label_clone.set_vertical_alignment(VerticalAlignment::Top);
        });
        view.add_subview(button.view);

        let label_clone = label.clone();
        let frame = Rectangle::new(PADDING, BUTTON_START + ((PADDING + BUTTON_HEIGHT as i32) * 4), INNER_WIDTH, BUTTON_HEIGHT);
        let button = Button::new(frame, "Middle", move || {
            label_clone.set_vertical_alignment(VerticalAlignment::Middle);
        });
        view.add_subview(button.view);

        let frame = Rectangle::new(PADDING, BUTTON_START + ((PADDING + BUTTON_HEIGHT as i32) * 5), INNER_WIDTH, BUTTON_HEIGHT);
        let button = Button::new(frame, "Bottom", move || {
            label.set_vertical_alignment(VerticalAlignment::Bottom);
        });
        view.add_subview(button.view);
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(200, 200, INNER_WIDTH + PADDING as u32 + PADDING as u32, 500);
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("Label example", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let application_main = ApplicationMain::new(AppDelegate {});
    application_main.launch();
    Ok(())
}

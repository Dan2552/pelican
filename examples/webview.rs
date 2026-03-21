use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color, WebView, TextField};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};

const URL_BAR_HEIGHT: u32 = 45;
const PADDING: u32 = 4;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        view.set_background_color(Color::new(48, 48, 48, 255));

        // URL bar
        let url_bar = TextField::new(
            Rectangle::new(
                PADDING as i32,
                PADDING as i32,
                WINDOW_WIDTH - (PADDING * 2),
                URL_BAR_HEIGHT,
            ),
            String::from("https://google.com"),
        );
        url_bar.set_background_color(Color::white());

        // WebView below the URL bar
        let web_view = WebView::new(Rectangle::new(
            PADDING as i32,
            (URL_BAR_HEIGHT + PADDING * 2) as i32,
            WINDOW_WIDTH - (PADDING * 2),
            WINDOW_HEIGHT - URL_BAR_HEIGHT - (PADDING * 3),
        ));
        web_view.load_url("https://google.com");

        view.add_subview(url_bar.clone());
        view.add_subview(web_view.clone());

        // Navigate when Return is pressed (inserts \n into text)
        url_bar.on_text_change(move |text_field| {
            let text = text_field.label().text().to_string();
            if text.contains('\n') {
                let mut url = text.trim().replace('\n', "");
                if !url.starts_with("http://") && !url.starts_with("https://") {
                    url = format!("https://{}", url);
                }
                text_field.label().set_text(url.clone());
                web_view.load_url(&url);
            }
        });
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(100, 100, WINDOW_WIDTH, WINDOW_HEIGHT);
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("Mini Browser", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let application_main = ApplicationMain::new(AppDelegate {});
    application_main.launch();
    Ok(())
}

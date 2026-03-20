use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::ScrollView;
use pelican::ui::button::Button;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        let frame = Rectangle::new(0, 0, 600, 400);
        let scroll_view = ScrollView::new(frame);

        // 5000x5000 content — would be a huge texture without clips_to_bounds
        let content_view = View::new(Rectangle::new(0, 0, 5000, 5000));
        content_view.set_background_color(Color::new(30, 30, 30, 255));

        // Grid of colored tiles so scrolling is visually obvious
        let colors = [
            Color::red(),
            Color::green(),
            Color::blue(),
            Color::new(255, 165, 0, 255), // orange
            Color::new(128, 0, 128, 255), // purple
            Color::new(0, 128, 128, 255), // teal
        ];

        let tile_size = 200;
        let cols = 5000 / tile_size;
        let rows = 5000 / tile_size;
        let mut color_idx = 0;

        for row in 0..rows {
            for col in 0..cols {
                let x = (col * tile_size) as i32;
                let y = (row * tile_size) as i32;

                let tile = View::new(Rectangle::new(
                    x + 2, y + 2,
                    tile_size as u32 - 4, tile_size as u32 - 4,
                ));
                tile.set_background_color(colors[color_idx % colors.len()].clone());
                content_view.add_subview(tile);

                color_idx += 1;
            }
        }

        // A few buttons scattered around to test interaction
        for (i, (bx, by)) in [(100, 100), (2500, 2500), (4800, 4800)].iter().enumerate() {
            let label = format!("Button {}", i + 1);
            let button = Button::new(
                Rectangle::new(*bx, *by, 120, 40),
                &label,
                move || { println!("Button {} tapped", i + 1); },
            );
            button.view.set_background_color(Color::white());
            content_view.add_subview(button.view);
        }

        scroll_view.set_content_view(content_view);
        view.add_subview(scroll_view.view);
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(200, 200, 600, 400);
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("Large scroll (5000x5000)", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let application_main = ApplicationMain::new(AppDelegate {});
    application_main.launch();
    Ok(())
}

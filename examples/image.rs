use pelican::graphics::{Rectangle, Point, Image};
use pelican::ui::{View, Window, ImageView};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        let tree_image = Image::new("tree.png");
        let tree = ImageView::new(tree_image, Point::new(10, 10));

        let always2tree = Image::new("always2tree.png");
        let tree2 = ImageView::new(always2tree, Point::new(350, 10));

        let always1tree = Image::new("always1tree.png");
        let tree3 = ImageView::new(always1tree, Point::new(700, 10));

        view.add_subview(tree.view);
        view.add_subview(tree2.view);
        view.add_subview(tree3.view);
    }
}


struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        // This is just to account for the example build directory. You don't
        // need to do this in your own code.
        copy_images();

        let frame = Rectangle::new(200, 200, 1100, 350);
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("Image example", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let application_main = ApplicationMain::new(AppDelegate {});
    application_main.launch();
    Ok(())
}

// This is just to account for the example build directory. You don't need to do
// this in your own code.
fn copy_images() {
    let mut from_dir = std::env::current_exe().unwrap();
    from_dir.pop();
    from_dir.pop();
    from_dir.pop();
    from_dir.pop();
    from_dir.push("examples");
    from_dir.push("resources");

    let mut target_dir = std::env::current_exe().unwrap();
    target_dir.pop();
    target_dir.push("resources");

    std::fs::create_dir_all(&target_dir).unwrap();

    let mut files = std::fs::read_dir(&from_dir).unwrap();
    while let Some(file) = files.next() {
        let file = file.unwrap();
        let path = file.path();
        let path = path.to_str().unwrap();
        std::fs::copy(path, &target_dir.join(file.file_name())).unwrap();
    }
}

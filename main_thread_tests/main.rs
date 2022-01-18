use pelican::ui::Window;
use pelican::ui::View;
use pelican::graphics::Rectangle;
use pelican::graphics::Point;
use pelican::graphics::Size;
use pelican::ui::{ViewController, ViewControllerBehavior};

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {}

pub fn main() -> Result<(), String> {
    println!("custom test: behavior");
    behavior();

    println!("custom test: parent_child_relationship");
    parent_child_relationship();
    Ok(())
}

fn behavior() {
    let frame = Rectangle {
        origin: Point { x: 10, y: 10 },
        size: Size { width: 50, height: 50 }
    };

    let view_controller = ViewController::new(ExampleViewController {});
    let window = Window::new("test", frame, view_controller);
    assert!(window.is_window());
}

/// Tests add_subview and superview
fn parent_child_relationship() {
    let frame = Rectangle {
        origin: Point { x: 10, y: 10 },
        size: Size { width: 50, height: 50 }
    };

    let view_controller = ViewController::new(ExampleViewController {});
    let view_parent = Window::new("test", frame.clone(), view_controller);
    let view_child = View::new(frame.clone());

    view_parent.add_subview(view_child.clone());

    let view_child1 = view_child.clone();
    let childs_parent = &view_child1.superview();

    assert_eq!(view_parent.view, childs_parent.upgrade().unwrap());

    let childs_parent_as_window = Window::from_window_view(childs_parent.upgrade().unwrap());
    assert_eq!(view_parent, childs_parent_as_window);

    let contains_child = view_parent.subviews().contains(&view_child);
    assert_eq!(contains_child, true);
}

use pelican::ui::Window;
use pelican::ui::View;
use pelican::graphics::Rectangle;
use pelican::graphics::Point;
use pelican::graphics::Size;

pub fn main() -> Result<(), String> {
    parent_child_relationship();
    Ok(())
}

// fn initialize() {
//     let frame = Rectangle {
//         position: Point { x: 10, y: 10 },
//         size: Size { width: 50, height: 50 }
//     };

//     let window = Window::new("test", frame);

//     {
//         let window_behavior = window.behavior.borrow();
//         let view = window_behavior.get_view();
//         assert_eq!(window, view.upgrade().unwrap());
//     }
// }

/// Tests add_subview and superview
fn parent_child_relationship() {
    let frame = Rectangle {
        position: Point { x: 10, y: 10 },
        size: Size { width: 50, height: 50 }
    };

    let mut view_parent = Window::new("test", frame.clone());
    let view_child = View::new(frame.clone());

    view_parent.add_subview(view_child.clone());

    let view_child1 = view_child.clone();
    let childs_parent = &view_child1.superview();

    assert_eq!(view_parent, childs_parent.upgrade().unwrap());

    let contains_child = view_parent.subviews().contains(&view_child);
    assert_eq!(contains_child, true);
}

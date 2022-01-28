use pelican::ui::Window;
use pelican::ui::View;
use pelican::graphics::Rectangle;
use pelican::graphics::Point;
use pelican::graphics::Size;
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::run_loop::RunLoop;
use pelican::ui::timer::Timer;
use pelican::ui::application::Application;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {}

pub fn main() -> Result<(), String> {
    println!("context");
    context();

    println!("custom test: behavior");
    behavior();

    println!("custom test: parent_child_relationship");
    parent_child_relationship();

    println!("custom test: application");
    application();
    Ok(())
}

fn context() {
    let frame = Rectangle {
        origin: Point { x: 10, y: 10 },
        size: Size { width: 50, height: 50 }
    };

    let view_controller = ViewController::new(ExampleViewController {});
    let window = Window::new("test", frame.clone(), view_controller);

    window.set_hidden(false);

    // Add a timer to run the loops once, to setup render layers
    window.set_needs_display();

    // Add a second timer to exit the run loop after the first iteration
    {
        let run_loop = RunLoop::borrow();
        // TODO: would this benefit from Window rather than View?
        let dirty_timer = Timer::new_once(move || {
            let run_loop = RunLoop::borrow();
            run_loop.exit();
        });
        run_loop.add_timer(dirty_timer);
    }

    let run_loop = RunLoop::borrow();
    run_loop.run();

    let layer = window.view.layer().unwrap();
    let context = layer.context();

    assert_eq!(window.view.frame().size(), layer.size());
    assert_eq!(layer.size(), &context.size());

    let scaled_width = layer.size().width as f32 * context.render_scale;
    let scaled_height = layer.size().height as f32 * context.render_scale;

    assert_eq!(scaled_width, context.pixel_size().width as f32);
    assert_eq!(scaled_height, context.pixel_size().height as f32);
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

    let childs_parent_as_window = Window::from_view(childs_parent.upgrade().unwrap());
    assert_eq!(view_parent, childs_parent_as_window);

    let contains_child = view_parent.subviews().contains(&view_child);
    assert_eq!(contains_child, true);
}

fn application() {
    let frame = Rectangle {
        origin: Point { x: 10, y: 10 },
        size: Size { width: 50, height: 50 }
    };

    let view_controller = ViewController::new(ExampleViewController {});
    let start_window_count: usize;
    {
        let application = Application::borrow();
        start_window_count = application.windows().len();
    }

    let _ = Window::new("test", frame, view_controller);
    let application = Application::borrow();
    assert_eq!(application.windows().len(), start_window_count + 1);
}

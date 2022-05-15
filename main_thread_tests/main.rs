use pelican::ui::Window;
use pelican::ui::View;
use pelican::graphics::Rectangle;
use pelican::graphics::Point;
use pelican::graphics::Size;
use pelican::graphics::Image;
use pelican::ui::ImageView;
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::run_loop::RunLoop;
use pelican::ui::timer::Timer;
use pelican::ui::application::Application;
use pelican::platform::thread;

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {}

pub fn main() -> Result<(), String> {
    println!("custom test: main thread");
    main_thread();

    println!("custom test: context");
    context();

    println!("custom test: behavior");
    behavior();

    println!("custom test: parent_child_relationship");
    parent_child_relationship();

    println!("custom test: scaled_images");
    scaled_images();

    println!("custom test: application");
    application();

    Ok(())
}

fn main_thread() {
    assert_eq!(thread::is_main(), true);
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

    let scaled_width = layer.size().width as f32 * context.render_scale();
    let scaled_height = layer.size().height as f32 * context.render_scale();

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

fn scaled_images() {
    let tree_image = Image::new(example_resources_directory().join("tree.png").to_str().unwrap());
    let tree = ImageView::new(tree_image, Point::new(10, 10));

    let always2tree = Image::new(example_resources_directory().join("always2tree.png").to_str().unwrap());
    let tree2 = ImageView::new(always2tree, Point::new(350, 10));

    let always1tree = Image::new(example_resources_directory().join("always1tree.png").to_str().unwrap());
    let tree3 = ImageView::new(always1tree, Point::new(700, 10));

    assert!(tree.view.frame().size() == tree2.view.frame().size());
    assert!(tree.view.frame().size() == tree3.view.frame().size());

    let frame = Rectangle {
        origin: Point { x: 10, y: 10 },
        size: Size { width: 50, height: 50 }
    };

    let view_controller = ViewController::new(ExampleViewController {});
    let window = Window::new("test", frame.clone(), view_controller);
    let context = window.context();

    let mut tree_image = Image::new(example_resources_directory().join("tree.png").to_str().unwrap());
    let layer0 = tree_image.layer_for(&context);

    let mut always2tree = Image::new(example_resources_directory().join("always2tree.png").to_str().unwrap());
    let layer2 = always2tree.layer_for(&context);

    let mut always1tree = Image::new(example_resources_directory().join("always1tree.png").to_str().unwrap());
    let layer1 = always1tree.layer_for(&context);

    assert_eq!(layer0.size(), layer1.size());
    assert_eq!(layer1.size(), layer2.size());

    let raw0 = layer0._raw_texture();
    let raw1 = layer1._raw_texture();
    let raw2 = layer2._raw_texture();

    if context.render_scale() == 1.0 {
        assert_eq!(raw0.query().width, raw1.query().width);
    } else {
        assert_eq!(raw0.query().width, raw2.query().width);
    }

    assert_ne!(raw1.query().width, raw2.query().width);
    assert_ne!(raw1.query().height, raw2.query().height);
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

fn example_resources_directory() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.pop();
    path.pop();
    path.push("examples/resources");
    path
}

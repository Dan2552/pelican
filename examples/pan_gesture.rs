use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::gesture::pan_recognizer::PanRecognizer;
use pelican::ui::gesture::pan_recognizer::PanState;
use std::cell::RefCell;
use pelican::graphics::Point;
use std::rc::Rc;

struct ExampleState {
    initial_center: Point<i32>
}

struct ExampleViewController {
    state: Rc<RefCell<ExampleState>>
}

impl ExampleViewController {
    fn new() -> ExampleViewController {
        ExampleViewController {
            state: Rc::new(RefCell::new(ExampleState {
                initial_center: Point::new(0, 0)
            }))
        }
    }
}

impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        let frame = Rectangle::new(0, 0, 100, 100);
        let red = View::new(frame);
        red.set_background_color(Color::red());
        view.add_subview(red.clone());

        let state = self.state.clone();
        let pan_gesture = PanRecognizer::new(move |gesture_recognizer| {
            let mut state = state.borrow_mut();

            if gesture_recognizer.view().is_none() {
                return;
            }

            let piece = gesture_recognizer.view().upgrade().unwrap();
            let superview = piece.superview().upgrade().unwrap();

            // Get the changes in the X and Y directions relative to
            // the superview's coordinate space.
            let translation = gesture_recognizer.translation_in(&superview);
            if gesture_recognizer.state() == PanState::Began {
                // Save the view's original position.
                state.initial_center = piece.frame().center();
            }

            let initial_center = state.initial_center.clone();

            // Update the position for the .began, .changed, and .ended states
            if gesture_recognizer.state() != PanState::Cancelled {
                // println!("updating position, translation: {:?}", translation);
                // Add the X and Y translation to the view's original position.
                let new_center = Point::new(initial_center.x + translation.x, initial_center.y + translation.y);
                piece.set_frame(Rectangle::new_from_center(new_center, piece.frame().size().clone()));
            } else {
                // On cancellation, return the piece to its original location.
                piece.set_frame(Rectangle::new_from_center(initial_center, piece.frame().size().clone()));
            }
        });
        red.add_gesture_recognizer(Box::new(pan_gesture));
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(200, 200, 400, 200);
        let view_controller = ViewController::new(ExampleViewController::new());
        let window = Window::new("Pan example", frame, view_controller);
        window.make_key_and_visible();
    }
}

pub fn main() -> Result<(), String> {
    let app_delegate = AppDelegate {};
    let application_main = ApplicationMain::new(Box::new(app_delegate));
    application_main.launch();
    Ok(())
}

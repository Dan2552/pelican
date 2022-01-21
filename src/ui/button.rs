use crate::macros::*;
use crate::ui::view::DefaultBehavior;
use crate::ui::{Touch, Label, Color};
use crate::graphics::Rectangle;
use std::cell::{Cell, RefCell};
use crate::text::{HorizontalAlignment, VerticalAlignment};

static DEFAULT_COLOR_NORMAL: Color = Color { red: 2, green: 117, blue: 227, alpha: 255 };
static DEFAULT_COLOR_PRESSED: Color = Color { red: 64, green: 155, blue: 255, alpha: 255 };

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Normal,
    Pressed
}

custom_view!(
    Button subclasses DefaultBehavior

    struct ButtonBehavior {
        state: Cell<State>,
        pressed_text_color: RefCell<Color>,
        last_normal_text_color: RefCell<Color>,
        action: Box<dyn Fn() -> ()>
    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>, text: &str, action: impl Fn() -> () + 'static) -> Button {
            let state = Cell::new(State::Normal);
            let button = Button::new_all(
                frame.clone(),
                state,
                RefCell::new(DEFAULT_COLOR_PRESSED.clone()),
                RefCell::new(DEFAULT_COLOR_NORMAL.clone()),
                Box::new(action)
            );
            let label_rectangle = Rectangle::new(0, 0, frame.size.width, frame.size.height);
            let label = Label::new(label_rectangle, String::from(text));
            label.set_text_color(DEFAULT_COLOR_NORMAL.clone());
            label.view.set_user_interaction_enabled(false);
            label.set_text_alignment(HorizontalAlignment::Center);
            label.set_vertical_alignment(VerticalAlignment::Middle);
            button.view.add_subview(label.view);
            button.view.set_background_color(Color::clear());
            button
        }

        fn label(&self) -> Label {
            let view = self.view.subviews().get(0).unwrap().clone();
            Label::from_view(view)
        }

        pub fn set_text_color(&self, color: Color) {
            self.label().set_text_color(color);
        }

        pub fn set_pressed_text_color(&self, color: Color) {
            let behavior = self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<ButtonBehavior>().unwrap();

            behavior.pressed_text_color.replace(color);
        }
    }

    impl Behavior {
        fn touches_began(&self, _touches: &Vec<Touch>) {
            self.set_state(State::Pressed);
        }

        fn touches_ended(&self, touches: &Vec<Touch>) {
            if let Some(touch) = touches.first() {

                let view = self.view.upgrade().unwrap();
                let window = touch.window().unwrap();

                let position = window.view.convert_point_to(&touch.position(), &view);

                if view.bounds().contains(&position) {
                    (self.action)();
                }
            }

            self.set_state(State::Normal);
        }

        fn touches_moved(&self, touches: &Vec<Touch>) {
            if let Some(touch) = touches.first() {
                let view = self.view.upgrade().unwrap();
                let window = touch.window().unwrap();

                let position = window.view.convert_point_to(&touch.position(), &view);

                if view.bounds().contains(&position) {
                    self.set_state(State::Pressed);
                } else {
                    self.set_state(State::Normal);
                }
            }
        }
    }
);

impl ButtonBehavior {
    fn set_state(&self, state: State) {
        let button = self.view_type();

        if self.state.get() == state {
            return;
        }

        match state {
            State::Normal => {
                button.label().set_text_color(self.last_normal_text_color.borrow().clone());
            },
            State::Pressed => {
                self.last_normal_text_color.replace(button.label().text_color());
                button.label().set_text_color(self.pressed_text_color.borrow().clone());
            }
        }

        self.state.set(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_new() {
        let button = Button::new(
            Rectangle::new(0, 0, 100, 100),
            "Test",
            Box::new(|| {})
        );

        assert_eq!(button.view.frame(), Rectangle::new(0, 0, 100, 100));
        assert_eq!(button.label().text(), String::from("Test"));
    }
}

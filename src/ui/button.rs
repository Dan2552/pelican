use crate::macros::*;
use crate::ui::view::DefaultBehavior;
use crate::ui::{Touch, Event, Label, View, Color};
use crate::graphics::Rectangle;
use std::cell::RefCell;

custom_view!(
    Button subclasses DefaultBehavior
    
    struct ButtonBehavior {}

    view impl {
        pub fn new(frame: Rectangle<i32, u32>, text: &str) -> Button {
            let button = Button::new_all(frame.clone());
            let label_rectangle = Rectangle::new(0, 0, frame.size.width, frame.size.height);
            let label = Label::new(label_rectangle, String::from(text));
            label.view.set_user_interaction_enabled(false);
            button.view.add_subview(label.view);
            button.view.set_background_color(Color::red());
            button
        }

        fn label(&self) -> View {
            self.view.get_subviews().get(0).unwrap().clone()
        }
    }

    behavior impl {
        fn touches_began(&self, touches: &Vec<Touch>, _event: Event) {
            println!("Button touches began");
        }

        fn touches_ended(&self, touches: &Vec<Touch>, _event: Event) {
            println!("Button touches ended");
            if let Some(touch) = touches.first() {
                
                let view = self.view.upgrade().unwrap();
                let window = touch.get_window().unwrap();
                
                let position = window.view.convert_point_to(&touch.get_position(), &view);

                if view.get_bounds().contains(&position) {
                    println!("Button clicked");
                } else {
                    println!("Button not clicked");
                }
            }
        }

        fn touches_moved(&self, touches: &Vec<Touch>, _event: Event) {
            println!("Button touches moved");
        }
    }
);

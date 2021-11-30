use crate::graphics::Rectangle;
use crate::graphics::Font;
use crate::graphics::Point;
use crate::graphics::Size;
use crate::ui::Color;
use crate::ui::{View, WeakView};
use crate::ui::view::{Behavior, DefaultBehavior};
use std::cell::RefCell;
use crate::macros::*;

custom_view!(
    Label subclasses DefaultBehavior 
    
    struct LabelBehavior {
        font: RefCell<Font<'static, 'static>>,
        text_color: Color,
        text: RefCell<String>
    }

    view impl {
        pub fn new(frame: Rectangle<i32, u32>, text: String) -> Label {
            let font = RefCell::new(Font::new("Arial", 17));
            let text_color = Color::black();
            let text = RefCell::new(text);
            let label = Self::new_all(frame, font, text_color, text);
            label.view.set_background_color(Color::clear());
            label
        }

        fn set_text(&self, text: String) {
            let behavior = &self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
            behavior.text.replace(text);
            behavior.set_needs_display();
        }

        // TODO:
        // font=(value)
        // text_color=(value)
        // text_alignment=(value)
        // number_of_lines=(value)
    }

    behavior impl {
        fn draw(&self) {
            self.super_behavior().unwrap().draw();

            let view = self.view.upgrade().unwrap().clone();
            let inner_self = view.inner_self.borrow();
            let behavior = view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();

            let text = behavior.text.borrow();

            if let Some(layer) = &inner_self.layer {
                let mut font = behavior.font.borrow_mut();
                let child_layer = font.layer_for(layer.context.clone(), &text);
                let position = Point { x: 0, y: 0 };
                let size = child_layer.get_size().clone();
                let size = Size {
                    width: size.width / 2,
                    height: size.height / 2
                };
                let destination = Rectangle { position, size };
                layer.draw_child_layer(&child_layer, &destination);
            }
        }
    }
);

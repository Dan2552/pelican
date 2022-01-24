use crate::graphics::{Rectangle, Size, Point};
use crate::ui::view::{View, WeakView};
use crate::ui::view::DefaultBehavior;
use crate::ui::Color;
use crate::macros::*;
use crate::ui::Label;
use std::cell::RefCell;

pub(crate) struct Carat {
    view: WeakView,
    character_index: usize
}

custom_view!(
    TextField subclasses DefaultBehavior

    struct TextFieldBehavior {
        carats: RefCell<Vec<Carat>>
    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>, text: String) -> TextField {
            let label = Label::new(frame.clone(), text);
            label.view.set_tag(1);

            let carats = RefCell::new(Vec::new());
            let text_field = TextField::new_all(frame, carats);

            text_field.view.add_subview(label.view);

            // text_field.spawn_carat(0);

            text_field
        }

        fn label(&self) -> Label {
            let view = self.view.view_with_tag(1).unwrap();
            Label::from_view(view)
        }

        pub fn spawn_carat(&self, character_index: usize) {
            let behavior = self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<TextFieldBehavior>().unwrap();
            let mut carats = behavior.carats.borrow_mut();

            let label_origin = &self.label().view.frame().origin;
            let origin: Point<i32>;
            if let Some(character_origin) = self.label().position_for_character_at_index(character_index) {
                // TODO: temporary hack. The 0.5 is to adjust for render scale.
                origin = Point {
                    x: (character_origin.x as f32 * 0.5) as i32 + label_origin.x - 1,
                    y: (character_origin.y as f32 * 0.5) as i32 + label_origin.y
                };
            } else {
                origin = Point {
                    x: label_origin.x - 1,
                    y: label_origin.y
                };
            }

            // TODO: line height
            let size = Size::new(2, 14);

            let carat_view = View::new(Rectangle { origin, size } );
            carat_view.set_background_color(Color::red());

            self.view.add_subview(carat_view.clone());

            let carat = Carat {
                view: carat_view.downgrade(),
                character_index
            };

            carats.push(carat);

            self.view.set_needs_display();
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;

}

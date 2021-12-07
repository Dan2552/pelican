use crate::graphics::{Rectangle, Font, Size};
use crate::ui::Color;
use crate::ui::view::{Behavior, DefaultBehavior};
use std::cell::{Cell, RefCell};
use crate::graphics::Layer;
use crate::text::attributed_string::AttributedString;
use crate::text::rendering;
use crate::macros::*;
use crate::text::{VerticalAlignment, HorizontalAlignment};

custom_view!(
    Label subclasses DefaultBehavior

    struct LabelBehavior {
        font: RefCell<Font>,
        text_color: RefCell<Color>,
        text: RefCell<String>,
        text_alignment: Cell<HorizontalAlignment>,
        text_vertical_alignment: Cell<VerticalAlignment>
    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>, text: String) -> Label {
            let font = RefCell::new(Font::default());
            let text_color = RefCell::new(Color::black());
            let text = RefCell::new(text);
            let text_alignment = Cell::new(HorizontalAlignment::Left);
            let text_vertical_alignment = Cell::new(VerticalAlignment::Top);

            let label = Self::new_all(
                frame,
                font,
                text_color,
                text,
                text_alignment,
                text_vertical_alignment
            );
            label.view.set_background_color(Color::clear());
            label
        }

        pub fn set_text(&self, text: String) {
            let behavior = &self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
            behavior.text.replace(text);
            behavior.set_needs_display();
        }

        pub fn set_text_color(&self, text_color: Color) {
            let behavior = &self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
            behavior.text_color.replace(text_color);
            behavior.set_needs_display();
        }

        pub fn get_text_color(&self) -> Color {
            let behavior = &self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
            let text_color = behavior.text_color.borrow();
            text_color.clone()
        }

        pub fn set_font(&self, font: Font) {
            let behavior = &self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
            behavior.font.replace(font);
            behavior.set_needs_display();
        }

        pub fn set_text_alignment(&self, text_alignment: HorizontalAlignment) {
            let behavior = &self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
            behavior.text_alignment.set(text_alignment);
            behavior.set_needs_display();
        }

        pub fn set_vertical_alignment(&self, text_vertical_alignment: VerticalAlignment) {
            let behavior = &self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
            behavior.text_vertical_alignment.set(text_vertical_alignment);
            behavior.set_needs_display();
        }

        /// Resizes the view's frame to fit the size of the text.
        pub fn fit_to_text(&self) {
            let behavior = &self.view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
            let text = behavior.text.borrow().clone();
            let font = behavior.font.borrow();

            let size = font.size_for(&text);

            let origin = self.view.frame().origin;
            let frame = Rectangle { origin, size };

            self.view.set_frame(frame);
        }


        // TODO:
        // number_of_lines=(value)
    }

    impl Behavior {
        fn draw(&self) {
            self.super_behavior().unwrap().draw();

            let view = self.view.upgrade().unwrap().clone();
            let inner_self = view.inner_self.borrow();

            let text = self.text.borrow();

            let attributed_string = AttributedString::new(text.clone());

            if let Some(parent_layer) = &inner_self.layer {
                let frame = view.frame();
                let mut whole_text = rendering::WholeText::from(&attributed_string, view.frame());
                // whole_text.align_horizontally(HorizontalAlignment::Right);
                // whole_text.align_vertically(VerticalAlignment::Bottom);

                for (character, position, attributed_substring) in whole_text.iter_characters_with_position() {
                    // let character_frame = Rectangle {
                    //     origin: position,
                    //     size: character.size().clone()
                    // };

                    // TODO: replace with attributed font
                    let font = self.font.borrow_mut();

                    // TODO: remove after debugging
                    let check = font.size_for(&character.to_string());
                    assert_eq!(&check, character.size());

                    // TODO: replace with attributed color
                    let color = self.text_color.borrow();
                    let color = color.to_graphics_color();

                    let child_layer = font.layer_for(
                        &parent_layer.context.clone(),
                        &character.to_string(),
                        color
                    );

                    // assert_eq!(child_layer.size().width / 2, character.size().width);

                    let size = child_layer.size();
                    let size = Size {
                        width: size.width,
                        height: size.height
                    };

                    let character_frame = Rectangle {
                        origin: position,
                        size: size
                    };

                    parent_layer.draw_child_layer_without_scaling(&child_layer, &character_frame);
                }
            }
        }
    }
);

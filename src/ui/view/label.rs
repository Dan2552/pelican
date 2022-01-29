use crate::graphics::{Rectangle, Font, Size};
use crate::ui::Color;
use crate::ui::view::{Behavior, DefaultBehavior};
use std::cell::{Cell, RefCell, Ref};
use crate::text::attributed_string::{AttributedString, Key, Attribute};
use crate::text::rendering;
use crate::macros::*;
use crate::text::{VerticalAlignment, HorizontalAlignment};
use std::ops::Range;

custom_view!(
    Label subclasses DefaultBehavior

    struct LabelBehavior {
        font: RefCell<Font>,
        text_color: RefCell<Color>,
        text: RefCell<String>,
        attributed_text: RefCell<Option<AttributedString>>,
        text_alignment: Cell<HorizontalAlignment>,
        text_vertical_alignment: Cell<VerticalAlignment>,
        rendering_result: RefCell<Option<rendering::Result>>
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
                RefCell::new(None),
                text_alignment,
                text_vertical_alignment,
                RefCell::new(None)
            );
            label.view.set_background_color(Color::clear());
            label
        }

        /// Returns a _copy_ of the text contained in the label.
        ///
        /// If the label contains an attributed string, the text is extracted
        /// from the attributed string.
        pub fn text(&self) -> String {
            let behavior = self.behavior();
            let attributed_text = behavior.attributed_text.borrow();

            if let Some(attributed_text) = attributed_text.as_ref() {
                let text = attributed_text.text();
                String::from(text)
            } else {
                let text = behavior.text.borrow();
                text.clone()
            }
        }

        pub fn set_text(&self, text: String) {
            let behavior = self.behavior();
            behavior.attributed_text.replace(None);
            behavior.text.replace(text);
            behavior.set_needs_display();
        }

        pub fn set_attributed_text(&self, attributed_text: AttributedString) {
            let behavior = self.behavior();
            behavior.text.replace("".to_string());
            behavior.attributed_text.replace(Some(attributed_text));
            behavior.set_needs_display();
        }

        pub fn insert_text_at_index(&self, index: usize, text_to_insert: &str) {
            let behavior = self.behavior();

            if behavior.attributed_text.borrow().is_some() {
                let mut attributed_text = behavior.attributed_text.borrow_mut();
                let attributed_text = attributed_text.as_mut().unwrap();
                attributed_text.insert_str(index, text_to_insert);
            } else {
                let mut text = behavior.text.borrow_mut();
                text.insert_str(index, text_to_insert);
            }

            behavior.set_needs_display();
        }

        pub fn replace_text_in_range(&self, range: Range<usize>, text_to_replace: &str) {
            let behavior = self.behavior();

            if behavior.attributed_text.borrow().is_some() {
                let mut attributed_text = behavior.attributed_text.borrow_mut();
                let attributed_text = attributed_text.as_mut().unwrap();
                attributed_text.replace_range(range, text_to_replace);
            } else {
                let mut text = behavior.text.borrow_mut();
                text.replace_range(range, text_to_replace);
            }

            behavior.set_needs_display();
        }

        pub fn set_text_color(&self, text_color: Color) {
            let behavior = self.behavior();
            behavior.text_color.replace(text_color);
            behavior.set_needs_display();
        }

        pub fn text_color(&self) -> Color {
            let behavior = self.behavior();
            let text_color = behavior.text_color.borrow();
            text_color.clone()
        }

        pub fn set_font(&self, font: Font) {
            let behavior = self.behavior();
            behavior.font.replace(font);
            behavior.set_needs_display();
        }

        pub fn font(&self) -> Font {
            let behavior = self.behavior();
            let font = behavior.font.borrow();
            font.clone()
        }

        pub fn set_text_alignment(&self, text_alignment: HorizontalAlignment) {
            let behavior = self.behavior();
            behavior.text_alignment.set(text_alignment);
            behavior.set_needs_display();
        }

        pub fn text_alignment(&self) -> HorizontalAlignment {
            let behavior = self.behavior();
            behavior.text_alignment.get()
        }

        pub fn set_vertical_alignment(&self, text_vertical_alignment: VerticalAlignment) {
            let behavior = self.behavior();
            behavior.text_vertical_alignment.set(text_vertical_alignment);
            behavior.set_needs_display();
        }

        /// Resizes the view's frame to fit the size of the text.
        pub fn fit_to_text(&self) {
            let behavior = self.behavior();
            let text = behavior.text.borrow().clone();
            let font = behavior.font.borrow();

            let size = font.size_for(&text);

            let origin = self.view.frame().origin;
            let frame = Rectangle { origin, size };

            self.view.set_frame(frame);
        }
    }

    impl Behavior {
        fn draw(&self) {
            self.super_behavior().unwrap().draw();

            let view = self.view.upgrade().unwrap().clone();
            let inner_self = view.inner_self.borrow();

            let behavior = view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();

            let text = self.text.borrow();
            let attributed_text = AttributedString::new(text.clone());
            attributed_text.set_default_attribute(
                Key::Font,
                Attribute::Font { font: behavior.font.borrow().clone() }
            );
            attributed_text.set_default_attribute(
                Key::Color,
                Attribute::Color { color: behavior.text_color.borrow().to_graphics_color() }
            );

            let mut attributed_text_ref = &attributed_text;
            let existing = self.attributed_text.borrow();
            if let Some(existing) = existing.as_ref() {
                attributed_text_ref = existing;
            }

            if let Some(parent_layer) = &inner_self.layer {
                let context = parent_layer.context();
                let mut whole_text = rendering::WholeText::from(attributed_text_ref, view.frame(), context.render_scale);
                whole_text.align_horizontally(self.text_alignment.get());
                whole_text.align_vertically(self.text_vertical_alignment.get());

                let rendering_result = whole_text.calculate_character_render_positions();

                for (index, character) in attributed_text.chars().enumerate() {
                    let font_attribute = attributed_text_ref.get_attribute_for(index, Key::Font);
                    let color_attribute = attributed_text_ref.get_attribute_for(index, Key::Color);
                    let font = font_attribute.font();
                    let color = color_attribute.color();
                    let position = rendering_result.position_for_character_at_index(index).unwrap();

                    let child_layer = font.layer_for(
                        &parent_layer.context.clone(),
                        &character.to_string(),
                        color.clone()
                    );

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

                self.rendering_result.replace(Some(rendering_result));
            }
        }
    }
);

impl LabelBehavior {
    pub fn rendering(&self) -> Ref<'_, Option<rendering::Result>> {
        Ref::map(self.rendering_result.borrow(), |rendering_result| rendering_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_text() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let label = Label::new(frame, String::from("Hello World"));

        assert_eq!(label.text(), String::from("Hello World"));

        label.set_text(String::from("Hello World 1"));

        assert_eq!(label.text(), String::from("Hello World 1"));
    }

    #[test]
    fn test_label_attributed_text() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let label = Label::new(frame, String::from("A"));

        let attributed_text = AttributedString::new(String::from("Hello World"));
        attributed_text.set_default_attribute(
            Key::Font,
            Attribute::Font { font: Font::default() }
        );
        attributed_text.set_default_attribute(
            Key::Color,
            Attribute::Color { color: sdl2::pixels::Color::BLACK }
        );
        label.set_attributed_text(attributed_text);
        assert_eq!(label.text(), String::from("Hello World"));
    }

    #[test]
    fn test_label_insert_text() {
        // When using plain text
        {
            let frame = Rectangle::new(0, 0, 100, 100);
            let label = Label::new(frame, String::from("two"));

            label.insert_text_at_index(0, &String::from("one "));

            assert_eq!(label.text(), String::from("one two"));
        }

        // When using attributed string
        {
            let frame = Rectangle::new(0, 0, 100, 100);
            let label = Label::new(frame, String::from("two"));

            let attributed_text = AttributedString::new(String::from("two"));
            label.set_attributed_text(attributed_text);

            label.insert_text_at_index(0, &String::from("one "));

            assert_eq!(label.text(), String::from("one two"));
        }
    }

    #[test]
    fn test_replace_text_in_range() {
        // When using plain text
        {
            let frame = Rectangle::new(0, 0, 100, 100);
            let label = Label::new(frame, String::from("one two three"));

            label.replace_text_in_range(4..7, &String::from("four"));

            assert_eq!(label.text(), String::from("one four three"));
        }

        // When using attributed string
        {
            let frame = Rectangle::new(0, 0, 100, 100);
            let label = Label::new(frame, String::from(""));

            let attributed_text = AttributedString::new(String::from("one two three"));
            label.set_attributed_text(attributed_text);

            label.replace_text_in_range(4..7, &String::from("four"));

            assert_eq!(label.text(), String::from("one four three"));
        }
    }

    #[test]
    fn test_label_text_color() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let label = Label::new(frame, String::from("A"));

        label.set_text_color(Color::black());

        assert_eq!(label.text_color(), Color::black());
    }

    #[test]
    fn test_label_font() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let label = Label::new(frame, String::from("A"));

        label.set_font(Font::new("Arial", 16));
        assert_eq!(label.font(), Font::new("Arial", 16));
    }

    #[test]
    fn test_label_text_alignment() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let label = Label::new(frame, String::from("A"));
        label.set_text_alignment(HorizontalAlignment::Right);
        assert_eq!(label.text_alignment(), HorizontalAlignment::Right);
    }
}

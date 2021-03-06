use crate::graphics::{Rectangle, Font, Size};
use crate::ui::Color;
use crate::ui::view::{Behavior, DefaultBehavior};
use std::cell::{Cell, RefCell, Ref};
use crate::text::attributed_string::{AttributedString, Key, Attribute};
use crate::text::rendering;
use crate::macros::*;
use crate::text::{VerticalAlignment, HorizontalAlignment};
use std::ops::Range;
use std::rc::Rc;
use crate::text::Text;

custom_view!(
    Label subclasses DefaultBehavior

    struct LabelBehavior {
        attributed_text: Rc<RefCell<AttributedString>>,
        text_alignment: Cell<HorizontalAlignment>,
        text_vertical_alignment: Cell<VerticalAlignment>,
        rendering_result: RefCell<Option<rendering::Result>>
    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>, text: String) -> Label {
            let text = Rc::new(RefCell::new(AttributedString::new(text)));
            let text_alignment = Cell::new(HorizontalAlignment::Left);
            let text_vertical_alignment = Cell::new(VerticalAlignment::Top);

            let label = Self::new_all(
                frame,
                text,
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
        pub fn copy_text(&self) -> String {
            let behavior = self.behavior();
            let attributed_text = behavior.attributed_text.borrow();
            String::from(attributed_text.text().string())
        }

        pub fn text(&self) -> &Text {
            let behavior = self.behavior();
            let attributed_text = behavior.attributed_text.clone();

            unsafe { attributed_text.as_ptr().as_ref().unwrap().text() }
        }

        pub fn text_len(&self) -> usize {
            self.behavior().attributed_text.borrow().len()
        }

        /// Sets the text and removes any styling currently set via
        /// `AttributedString`.
        pub fn set_text(&self, text: String) {
            let behavior = self.behavior();
            let attributed_text: AttributedString;
            {
                let existing = behavior.attributed_text.borrow();
                // TODO: spec
                attributed_text = AttributedString::new_matching_default_style(text, &existing);
            }
            (*behavior.attributed_text).replace(attributed_text);
            behavior.set_needs_display();
        }

        pub fn set_attributed_text(&self, attributed_text: AttributedString) {
            let behavior = self.behavior();
            (*behavior.attributed_text).replace(attributed_text);
            behavior.set_needs_display();
        }

        pub fn insert_text_at_index(&self, index: usize, text_to_insert: &str) {
            let behavior = self.behavior();

            {
                let mut attributed_text = behavior.attributed_text.borrow_mut();
                attributed_text.insert_str(index, text_to_insert);
            }

            behavior.set_needs_display();
        }

        pub fn replace_text_in_range(&self, range: Range<usize>, text_to_replace: &str) {
            let behavior = self.behavior();

            {
                let mut attributed_text = behavior.attributed_text.borrow_mut();
                attributed_text.replace_range(range, text_to_replace);
            }

            behavior.set_needs_display();
        }

        pub fn set_text_color(&self, text_color: Color) {
            let behavior = self.behavior();

            {
                let attributed_text = behavior.attributed_text.borrow();
                attributed_text.set_default_attribute(
                    Key::Color,
                    Attribute::Color { color: text_color.to_graphics_color() }
                );
            }

            behavior.set_needs_display();
        }

        pub fn text_color(&self) -> Color {
            let behavior = self.behavior();

            let attributed_text = behavior.attributed_text.borrow();
            let attribute = attributed_text.default_attribute(Key::Color);
            let graphics_color = attribute.color();

            Color::from_graphics_color(graphics_color)
        }

        pub fn set_font(&self, font: Font) {
            let behavior = self.behavior();

            {
                let attributed_text = behavior.attributed_text.borrow();
                attributed_text.set_default_attribute(
                    Key::Font,
                    Attribute::Font { font }
                );
            }

            behavior.set_needs_display();
        }

        pub fn font(&self) -> Font {
            let behavior = self.behavior();

            let attributed_text = behavior.attributed_text.borrow();
            let attribute = attributed_text.default_attribute(Key::Font);
            attribute.font().clone()
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
            // TODO: add size to rendering_result

            // let behavior = self.behavior();
            // let text = behavior.text.borrow().clone();
            // let font = behavior.font.borrow();

            // let size = font.size_for(&text);

            // let origin = self.view.frame().origin;
            // let frame = Rectangle { origin, size };

            // self.view.set_frame(frame);
        }

        fn generate_rendering_result(&self) {
            let inner_self = self.view.inner_self.borrow();
            let behavior = self.behavior();
            let attributed_string = behavior.attributed_text.borrow();

            let render_scale: f32;
            if let Some(parent_layer) = &inner_self.layer {
                let context = parent_layer.context();
                render_scale = context.render_scale()
            } else {
                render_scale = 1.0;
            }

            let mut whole_text = rendering::WholeText::from(&attributed_string, self.view.frame(), render_scale);
            whole_text.align_horizontally(behavior.text_alignment.get());
            whole_text.align_vertically(behavior.text_vertical_alignment.get());

            let rendering_result = whole_text.calculate_character_render_positions();
            behavior.rendering_result.replace(Some(rendering_result));
        }
    }

    impl Behavior {
        fn set_needs_display(&self) {
            self.super_behavior().unwrap().set_needs_display();
            let label = Label::from_view(self.view.upgrade().unwrap());
            label.generate_rendering_result();
        }

        fn draw(&self) {
            self.super_behavior().unwrap().draw();
            let label = Label::from_view(self.view.upgrade().unwrap());



            let view = self.view.upgrade().unwrap().clone();
            let inner_self = view.inner_self.borrow();

            let mut needs_generation = false;

            if let Some(rendering_result) = self.rendering_result.borrow().as_ref() {
                if let Some(parent_layer) = &inner_self.layer {
                    let context = parent_layer.context();

                    if context.render_scale() != rendering_result.render_scale() {
                        needs_generation = true;
                    }
                }
            } else {
                needs_generation = true;
            };

            if needs_generation {
                label.generate_rendering_result();
            }

            let attributed_string = self.attributed_text.borrow();

            if let Some(parent_layer) = &inner_self.layer {
                let rendering_result = self.rendering_result.borrow();
                let rendering_result = rendering_result.as_ref().unwrap();

                for (index, character) in attributed_string.chars().enumerate() {
                    let font_attribute = &attributed_string.get_attribute_for(index, Key::Font);
                    let color_attribute = &attributed_string.get_attribute_for(index, Key::Color);
                    let font = font_attribute.font();
                    let color = color_attribute.color();
                    let position = rendering_result.position_for_character_at_index(index);

                    let child_layer = font.layer_for(
                        parent_layer.context(),
                        &character.to_string(),
                        color.clone()
                    );

                    let size = child_layer.size();
                    let size = Size {
                        width: size.width,
                        height: size.height
                    };

                    let character_frame = Rectangle {
                        origin: position.clone(),
                        size: size
                    };

                    parent_layer.draw_child_layer_without_scaling(&child_layer, &character_frame);
                }
            }
        }
    }
);

impl LabelBehavior {
    pub fn rendering(&self) -> Ref<'_, rendering::Result> {
        let label = Label::from_view(self.view.upgrade().unwrap());
        let rendering_result = self.rendering_result.borrow();
        if rendering_result.is_none() {
            label.generate_rendering_result();
        };

        Ref::map(self.rendering_result.borrow(), |rendering_result| rendering_result.as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_text() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let label = Label::new(frame, String::from("Hello World"));

        assert_eq!(label.copy_text(), String::from("Hello World"));

        label.set_text(String::from("Hello World 1"));

        assert_eq!(label.copy_text(), String::from("Hello World 1"));
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
        assert_eq!(label.copy_text(), String::from("Hello World"));
    }

    #[test]
    fn test_label_insert_text() {
        // When using plain text
        {
            let frame = Rectangle::new(0, 0, 100, 100);
            let label = Label::new(frame, String::from("two"));

            label.insert_text_at_index(0, &String::from("one "));

            assert_eq!(label.copy_text(), String::from("one two"));
        }

        // When using attributed string
        {
            let frame = Rectangle::new(0, 0, 100, 100);
            let label = Label::new(frame, String::from("two"));

            let attributed_text = AttributedString::new(String::from("two"));
            label.set_attributed_text(attributed_text);

            label.insert_text_at_index(0, &String::from("one "));

            assert_eq!(label.copy_text(), String::from("one two"));
        }
    }

    #[test]
    fn test_replace_text_in_range() {
        // When using plain text
        {
            let frame = Rectangle::new(0, 0, 100, 100);
            let label = Label::new(frame, String::from("one two three"));

            label.replace_text_in_range(4..7, &String::from("four"));

            assert_eq!(label.copy_text(), String::from("one four three"));
        }

        // When using attributed string
        {
            let frame = Rectangle::new(0, 0, 100, 100);
            let label = Label::new(frame, String::from(""));

            let attributed_text = AttributedString::new(String::from("one two three"));
            label.set_attributed_text(attributed_text);

            label.replace_text_in_range(4..7, &String::from("four"));

            assert_eq!(label.copy_text(), String::from("one four three"));
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

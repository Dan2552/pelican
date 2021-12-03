use crate::graphics::{Rectangle, Font, Point, Size};
use crate::ui::Color;
use crate::ui::view::{Behavior, DefaultBehavior};
use std::cell::{Cell, RefCell};
use crate::graphics::Layer;
use crate::macros::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom
}

custom_view!(
    Label subclasses DefaultBehavior 
    
    struct LabelBehavior {
        font: RefCell<Font<'static, 'static>>,
        text_color: RefCell<Color>,
        text: RefCell<String>,
        text_alignment: Cell<HorizontalAlignment>,
        text_vertical_alignment: Cell<VerticalAlignment>
    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>, text: String) -> Label {
            let font = RefCell::new(Font::new("Arial", 17));
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

        pub fn set_font(&self, font: Font<'static, 'static>) {
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
            
            let size = font.size_of(&text);

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
            let behavior = view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();

            let text = behavior.text.borrow();

            if let Some(layer) = &inner_self.layer {
                for (index, line) in text.lines().enumerate() {    
                    self.draw_line(layer, line, index);
                }
            }
        }
    }
);

impl LabelBehavior {
    fn draw_line(&self, parent_layer: &Layer, text: &str, index: usize) {
        let mut font = self.font.borrow_mut();
        let color = self.text_color.borrow();
        let color = color.to_graphics_color();
        let child_layer = font.layer_for(parent_layer.context.clone(), text, color);
        
        let size = child_layer.get_size().clone();
        let size = Size {
            width: size.width / 2,
            height: size.height / 2
        };

        let x = match self.text_alignment.get() {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Center => (parent_layer.get_size().width as i32 - size.width as i32) / 2,
            HorizontalAlignment::Right => parent_layer.get_size().width as i32 - size.width as i32
        };
        
        let y = match self.text_vertical_alignment.get() {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => parent_layer.get_size().height as i32 / 2 - size.height as i32 / 2,
            VerticalAlignment::Bottom => parent_layer.get_size().height as i32 - size.height as i32
        };

        let origin = Point {
            x: x,
            y: y
        };

        let destination = Rectangle { origin, size };
        parent_layer.draw_child_layer(&child_layer, &destination);
    }
}

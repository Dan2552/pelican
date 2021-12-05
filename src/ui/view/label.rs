use crate::graphics::{Rectangle, Font};
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

            if let Some(layer) = &inner_self.layer {
                self.draw_text(layer, &text);
            }
        }
    }
);

impl LabelBehavior {
    fn draw_text(&self, layer: &Layer, text: &str) {
        // let context = layer.context();
        // let default_font = self.font.borrow();
        // let default_text_color = self.text_color.borrow();

        // let mut whole_width = 0;
        // let mut whole_height = 0;

        // let mut line_layers: Vec<(Point<i32>, Layer)> = Vec::new();

        // let mut line_y = 0;

        // // For each line - first build up all the layers and measure them, but
        // // they cannot be drawn yet until the size of the whole label is known.
        // for line in text.lines() {
        //     let mut line_width = 0;
        //     let mut max_line_height = 0;
        //     let mut character_layers: Vec<(Point<i32>, Layer)> = Vec::new();

        //     // For each character - first build up all the layers and measure
        //     // them, but they cannot be drawn yet until the size of the whole
        //     // line is known.
        //     for character in line.chars() {
        //         let font = &default_font;
        //         let text_color = &default_text_color;
        //         let character = &String::from(character);

        //         let character_layer = font.layer_for(&context, character, text_color.to_graphics_color());
        //         let point = Point::new(line_width as i32, line_y as i32);
        //         line_width = line_width + character_layer.size().width;
        //         max_line_height = max_line_height.max(character_layer.size().height);
        //         character_layers.push((point, character_layer));
        //     }

        //     let line_layer = Layer::new_no_render(
        //         context.clone(),
        //         Size::new(line_width, max_line_height)
        //     );

        //     for character_layer in character_layers {
        //         let (point, layer) = character_layer;
        //         let destination = Rectangle {
        //             origin: point,
        //             size: layer.size().clone()
        //         };
        //         line_layer.draw_child_layer(&layer, &destination);
        //     }

        //     let point = Point::new(0, line_y);
        //     line_layers.push((point, line_layer));

        //     whole_width = whole_width + line_width;
        //     whole_height = whole_height + max_line_height;
        //     line_y = line_y + max_line_height;
        // }

        // let whole_layer = Layer::new_no_render(
        //     context.clone(),
        //     Size::new(whole_width, whole_height)
        // );



        // TODO: text align final result
    }

    // fn draw_text(&self, layer: &Layer, text: &str) {
    //     let x = match self.text_alignment.get() {
    //         HorizontalAlignment::Left => 0,
    //         HorizontalAlignment::Center => (layer.get_size().width as i32 - size.width as i32) / 2,
    //         HorizontalAlignment::Right => layer.get_size().width as i32 - size.width as i32
    //     };

    //     let y = match self.text_vertical_alignment.get() {
    //         VerticalAlignment::Top => 0,
    //         VerticalAlignment::Center => layer.get_size().height as i32 / 2 - size.height as i32 / 2,
    //         VerticalAlignment::Bottom => layer.get_size().height as i32 - size.height as i32
    //     };

    //     let origin = Point {
    //         x: x,
    //         y: y
    //     };

    //     for (index, line) in text.lines().enumerate() {
    //         self.draw_line(layer, line, index);
    //     }
    // }

    // fn draw_line(&self, parent_layer: &Layer, text: &str, index: usize) {
    //     let mut font = self.font.borrow_mut();
    //     let color = self.text_color.borrow();
    //     let color = color.to_graphics_color();
    //     let child_layer = font.layer_for(parent_layer.context.clone(), text, color);

    //     let size = child_layer.get_size().clone();
    //     let size = Size {
    //         width: size.width / 2,
    //         height: size.height / 2
    //     };



    //     let destination = Rectangle { origin, size };
    //     parent_layer.draw_child_layer(&child_layer, &destination);
    // }
}

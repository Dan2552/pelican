use crate::graphics::{Rectangle, Size, Point};
use crate::ui::view::{View, WeakView};
use crate::ui::view::DefaultBehavior;
use crate::ui::Color;
use crate::macros::*;
use crate::ui::Label;
use crate::ui::run_loop::RunLoop;
use crate::ui::timer::Timer;
use std::cell::RefCell;
use std::time::Duration;

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

            text_field.spawn_carat(0);

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


            // The frame doesn't matter here, it will be updated later when the
            // view draws.
            let carat_view = View::new(Rectangle::new(0, 0, 1, 1));

            carat_view.set_background_color(Color::red());
            carat_view.set_hidden(true);

            self.view.add_subview(carat_view.clone());

            let carat = Carat {
                view: carat_view.downgrade(),
                character_index
            };

            carats.push(carat);

            self.view.set_needs_display();

            let weak_view = carat_view.downgrade();

            let timer = Timer::new_repeating(Duration::from_millis(500), move || {
                if let Some(view) = weak_view.upgrade() {
                    view.set_hidden(!view.is_hidden());
                } else {
                    // TODO: end this timer when the view is destroyed
                    panic!("view was destroyed");
                }
            });
            let run_loop = RunLoop::borrow();
            run_loop.add_timer(timer);
        }

        /// The cursors need repositioning when the view draws. This is because
        /// certain aspecs rely on the rendering layer, of which will not be
        /// present yet until this view is in the view hierarchy belonging to a
        /// window. Or the line of text that the cursor is sized on have have
        /// changed size.
        fn position_cursors(&self) {
            let label = self.label();
            let label_behavior = label.behavior();

            let behavior = self.behavior();
            let carats = behavior.carats.borrow();
            let layer = self.view.layer().unwrap();
            let render_scale = layer.context().render_scale;

            let rendering = label_behavior.rendering();

            if rendering.is_none() {
                for carat in carats.iter() {
                    let carat_view = carat.view.upgrade().unwrap();
                    carat_view.set_hidden(true);
                }
                return;
            }

            let rendering = rendering.as_ref().unwrap();

            for carat in carats.iter() {
                let character_index = carat.character_index;
                let label_origin = &self.label().view.frame().origin;
                let carat_view = carat.view.upgrade().unwrap();
                let cursor_rectangle = rendering.cursor_rectangle_for_character_at_index(character_index);

                let cursor_rectangle = Rectangle {
                    origin: Point {
                        x: (cursor_rectangle.origin.x as f32 / render_scale).round() as i32 + label_origin.x,
                        y: (cursor_rectangle.origin.y as f32 / render_scale).round() as i32 + label_origin.y
                    },
                    size: Size {
                        width: (cursor_rectangle.size.width as f32 / render_scale).round() as u32,
                        height: (cursor_rectangle.size.height as f32 / render_scale).round() as u32
                    }
                };

                carat_view.set_frame(cursor_rectangle);
            }
        }
    }

    impl Behavior {
        fn draw(&self) {
            self.super_behavior().unwrap().draw();
            let text_field = TextField::from_view(self.get_view().upgrade().unwrap());
            text_field.position_cursors();
        }
    }
);

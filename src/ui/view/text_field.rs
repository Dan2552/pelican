use crate::graphics::{Rectangle, Size, Point};
use crate::ui::view::{View, WeakView};
use crate::ui::view::DefaultBehavior;
use crate::ui::Color;
use crate::macros::*;
use crate::ui::Label;
use crate::ui::run_loop::RunLoop;
use crate::ui::timer::Timer;
use crate::ui::touch::Touch;
use std::cell::RefCell;
use std::time::Duration;

pub(crate) struct Carat {
    view: WeakView,
    character_index: usize,
    selection: Option<Selection>
}

pub(crate) struct Selection {
    start: usize,
    end: usize,
    views: RefCell<Vec<WeakView>>
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
            label.view.set_user_interaction_enabled(false);
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

        pub fn select(&self, cursor: usize, start: usize, end: usize) {
            let render_scale: f32;
            {
                let behavior = self.behavior();
                let mut carats = behavior.carats.borrow_mut();
                let carat = carats.get_mut(cursor).unwrap();
                carat.selection = Some(Selection {
                    start,
                    end,
                    views: RefCell::new(Vec::new())
                });

                let layer = self.view.layer().unwrap();
                render_scale = layer.context().render_scale;
            }
            let behavior = self.behavior();
            let carats = behavior.carats.borrow();
            let carat = carats.get(cursor).unwrap();
            self.position_selection(&carat.selection.as_ref().unwrap(), render_scale);
        }

        pub fn remove_carats(&self) {
            let behavior = self.behavior();
            let mut carats = behavior.carats.borrow_mut();
            for carat in carats.iter() {
                carat.view.upgrade().unwrap().remove_from_superview();
            }
            carats.clear();
        }

        pub fn spawn_carat(&self, character_index: usize) {
            let behavior = self.behavior();
            let mut carats = behavior.carats.borrow_mut();


            // The frame doesn't matter here, it will be updated later when the
            // view draws.
            let carat_view = View::new(Rectangle::new(0, 0, 1, 1));

            carat_view.set_background_color(Color::new(226, 175, 10, 255));
            carat_view.set_hidden(true);
            carat_view.set_user_interaction_enabled(false);
            self.view.add_subview(carat_view.clone());

            let carat = Carat {
                view: carat_view.downgrade(),
                character_index,
                selection: None
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
        /// certain aspects rely on the rendering layer, of which will not be
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
                        x: (cursor_rectangle.origin.x as f32 / render_scale).round() as i32 + label_origin.x - 1,
                        y: (cursor_rectangle.origin.y as f32 / render_scale).round() as i32 + label_origin.y
                    },
                    size: Size {
                        width: 2,
                        height: (cursor_rectangle.size.height as f32 / render_scale).round() as u32
                    }
                };

                carat_view.set_frame(cursor_rectangle);
            }
        }

        fn position_selection(&self, selection: &Selection, render_scale: f32) {
            // start: usize,
            // end: usize,
            // views: Vec<WeakView>

            let label = self.label();
            let label_behavior = label.behavior();
            let rendering = label_behavior.rendering();
            let rendering = rendering.as_ref().unwrap();

            for view in selection.views.borrow().iter() {
                let view = view.upgrade().unwrap();
                view.remove_from_superview();
            }

            selection.views.borrow_mut().clear();

            let label_origin = &self.label().view.frame().origin;

            let mut character_index = selection.start;
            let mut last_y = -9999999;
            let mut current_view: View = View::new(Rectangle::new(0, 0, 1, 1));
            loop {
                if character_index >= selection.end {
                    break;
                }

                let cursor_rectangle = rendering.cursor_rectangle_for_character_at_index(character_index);
                let character_size = rendering.character_size_for_character_at_index(character_index);

                if cursor_rectangle.origin.y != last_y {
                    last_y = cursor_rectangle.origin.y;
                    current_view = View::new(Rectangle::new(0, 0, 1, 1));
                    current_view.set_user_interaction_enabled(false);
                    self.view.add_subview(current_view.clone());
                    current_view.set_frame(Rectangle {
                        origin: Point {
                            x: (cursor_rectangle.origin.x as f32 / render_scale).round() as i32 + label_origin.x,
                            y: (cursor_rectangle.origin.y as f32 / render_scale).round() as i32 + label_origin.y
                        },
                        size: Size {
                            width: 0,
                            height: (cursor_rectangle.size.height as f32 / render_scale).round() as u32
                        }
                    });
                    current_view.set_background_color(Color::new(226, 175, 10, 64));
                }

                let current_frame = current_view.frame();

                current_view.set_frame(Rectangle::new(
                    current_frame.origin.x,
                    current_frame.origin.y,
                    current_frame.size.width + (character_size.width as f32 / render_scale).round() as u32,
                    current_frame.size.height
                ));

                character_index += 1;
            }
        }
    }

    impl Behavior {
        fn draw(&self) {
            self.super_behavior().unwrap().draw();
            let text_field = TextField::from_view(self.get_view().upgrade().unwrap());
            text_field.position_cursors();
        }

        fn touches_ended(&self, touches: &Vec<Touch>) {
            println!("touches ended");
            let view = self.view.upgrade().unwrap();
            let text_field = TextField::from_view(view.clone());

            let character_index: usize;
            {
                let touch = touches.first().unwrap();
                let window = touch.window().unwrap();
                let label = text_field.label();
                let position = window.view.convert_point_to(&touch.position(), &label.view);
                let label_behavior = label.behavior();
                let rendering = label_behavior.rendering();
                let rendering = rendering.as_ref().unwrap();
                let layer = view.layer().unwrap();
                let render_scale = layer.context().render_scale;

                let position = Point {
                    x: (position.x as f32 * render_scale).round() as i32,
                    y: (position.y as f32 * render_scale).round() as i32
                };

                character_index = rendering.character_at_position(position).unwrap_or(label.text().len());
            }

            text_field.remove_carats();
            text_field.spawn_carat(character_index);
        }
    }
);

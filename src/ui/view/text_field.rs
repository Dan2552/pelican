use crate::graphics::{Rectangle, Size, Point};
use crate::ui::view::{View, WeakView};
use crate::ui::view::DefaultBehavior;
use crate::ui::Color;
use crate::macros::*;
use crate::ui::Label;
use crate::ui::run_loop::RunLoop;
use crate::ui::timer::Timer;
use crate::ui::touch::Touch;
use crate::ui::press::Press;
use crate::ui::key::{KeyCode, ModifierFlag};
use std::cell::RefCell;
use std::time::Duration;
use std::cell::Cell;
use std::rc::Rc;
use std::ops::Range;

pub(crate) struct Carat {
    view: WeakView,
    character_index: Cell<usize>,
    selection: Option<Selection>,
    delay_animation: Rc<Cell<bool>>
}

pub(crate) struct Selection {
    start: usize,
    end: usize,
    views: RefCell<Vec<WeakView>>
}

custom_view!(
    TextField subclasses DefaultBehavior

    struct TextFieldBehavior {
        carats: RefCell<Vec<Carat>>,

        // A count of how many times the user is holding shift. This is a
        // count rather than a boolean because left and right shift are
        // handled as different key codes.
        holding_shift: Cell<u8>,

        // A count of how many times the user is holding control. This is a
        // count rather than a boolean because left and right alt keys are
        // handled as different key codes.
        holding_alternative: Cell<u8>,

        // When holding down touch, as the user moves their finger, a highlight
        // is made from where the touch started to where the finger is now.
        touch_began_at_index: Cell<usize>
    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>, text: String) -> TextField {
            let label = Label::new(frame.clone(), text);
            label.view.set_tag(1);
            label.view.set_user_interaction_enabled(false);
            let carats = RefCell::new(Vec::new());
            let text_field = TextField::new_all(
                frame,
                carats,
                Cell::new(0),
                Cell::new(0),
                Cell::new(0)
            );

            text_field.view.add_subview(label.view);

            text_field.spawn_carat(0);

            text_field
        }

        fn label(&self) -> Label {
            let view = self.view.view_with_tag(1).unwrap();
            Label::from_view(view)
        }

        fn touch_to_index(&self, touch: &Touch) -> usize {
            let window = touch.window().unwrap();
            let label = self.label();
            let position = window.view.convert_point_to(&touch.position(), &label.view);
            let label_behavior = label.behavior();
            let rendering = label_behavior.rendering();
            let rendering = rendering.as_ref().unwrap();
            let render_scale = rendering.render_scale();

            let position = Point {
                x: (position.x as f32 * render_scale).round() as i32,
                y: (position.y as f32 * render_scale).round() as i32
            };

            rendering.character_at_position(position).unwrap_or(label.text().len())
        }

        fn select(&self, carat: &mut Carat, index_one: usize, index_two: usize) {
            let lhs: usize;
            let rhs: usize;

            if index_one < index_two {
                lhs = index_one;
                rhs = index_two;
            } else {
                lhs = index_two;
                rhs = index_one;
            }

            self.select_range(carat, lhs..rhs);
        }

        fn select_range(&self, carat: &mut Carat, range: Range<usize>) {
            {
                if let Some(existing_selection) = &carat.selection {
                    for view in existing_selection.views.borrow().iter() {
                        let view = view.upgrade().unwrap();
                        view.remove_from_superview();
                    }
                }
            }

            if range.is_empty() {
                carat.selection = None;
                return;
            }

            carat.selection = Some(Selection {
                start: range.start,
                end: range.end,
                views: RefCell::new(Vec::new())
            });

            self.position_selection(&carat.selection.as_ref().unwrap());
        }

        pub fn remove_carats(&self) {
            let behavior = self.behavior();
            let mut carats = behavior.carats.borrow_mut();
            for carat in carats.iter() {
                carat.view.upgrade().unwrap().remove_from_superview();
                if let Some(selection) = &carat.selection {
                    for view in selection.views.borrow().iter() {
                        if let Some(view) = view.upgrade() {
                            view.remove_from_superview();
                        }
                    }
                }
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
                character_index: Cell::new(character_index),
                selection: None,
                delay_animation: Rc::new(Cell::new(false))
            };
            let delay_animation = Rc::downgrade(&carat.delay_animation);
            carats.push(carat);

            self.view.set_needs_display();

            let weak_view = carat_view.downgrade();

            let timer = Timer::new_repeating(Duration::from_millis(500), move || {
                if let Some(view) = weak_view.upgrade() {
                    if let Some(delay_animation) = delay_animation.upgrade() {
                        if delay_animation.get() {
                            view.set_hidden(false);
                            delay_animation.set(false);
                        } else {
                            view.set_hidden(!view.is_hidden());
                        }
                    }
                } else {
                    // TODO: end this timer when the view is destroyed
                    panic!("view was destroyed");
                }
            });
            let run_loop = RunLoop::borrow();
            run_loop.add_timer(timer);
        }

        // TODO: is this still correct?
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
            let rendering = label_behavior.rendering();

            if rendering.is_none() {
                for carat in carats.iter() {
                    let carat_view = carat.view.upgrade().unwrap();
                    carat_view.set_hidden(true);
                }
                return;
            }

            let rendering = rendering.as_ref().unwrap();

            let render_scale = rendering.render_scale();

            for carat in carats.iter() {
                let character_index = carat.character_index.get();
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

        fn position_selection(&self, selection: &Selection) {
            let label = self.label();
            let label_behavior = label.behavior();
            let rendering = label_behavior.rendering();
            let rendering = rendering.as_ref().unwrap();
            let render_scale = rendering.render_scale();

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

                selection.views.borrow_mut().push(current_view.downgrade());

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

        fn touches_moved(&self, touches: &Vec<Touch>) {
            let view = self.view.upgrade().unwrap();
            let text_field = TextField::from_view(view.clone());

            if let Some(last_cursor) = self.carats.borrow_mut().last_mut() {
                let start = self.touch_began_at_index.get();
                let touched_character_index = text_field.touch_to_index(touches.first().unwrap());

                text_field.select(last_cursor, start, touched_character_index);
                last_cursor.character_index.set(touched_character_index);
            }
        }

        fn touches_began(&self, touches: &Vec<Touch>) {
            let view = self.view.upgrade().unwrap();
            let text_field = TextField::from_view(view.clone());

            let touched_character_index = text_field.touch_to_index(touches.first().unwrap());

            self.touch_began_at_index.set(touched_character_index);

            if self.holding_shift.get() > 0 {
                let behavior = text_field.behavior();
                for cursor in behavior.carats.borrow_mut().iter_mut() {
                    let cursor_index = cursor.character_index.get();

                    text_field.select(cursor, cursor_index, touched_character_index);

                    cursor.character_index.set(touched_character_index);

                    // We only actually care to do this for one cursor; it's an
                    // edge case that we don't want to handle if there's more
                    // than one.
                    return;
                }
            }

            text_field.remove_carats();
            text_field.spawn_carat(touched_character_index);
        }

        fn text_input_did_receive(&self, text: &str) {
            let view = self.view.upgrade().unwrap();
            let text_field = TextField::from_view(view.clone());
            let label = text_field.label();
            let text_field_behavior = text_field.behavior();

            let carats = text_field_behavior.carats.borrow();

            for carat in carats.iter() {
                let index = carat.character_index.get();
                label.insert_text_at_index(index, text);
                carat.character_index.set(index + text.len());
                if let Some(carat_view) = carat.view.upgrade() {
                    carat_view.set_hidden(false);
                    carat_view.set_needs_display();
                }
                carat.delay_animation.set(true);
            }
        }

        fn press_ended(&self, press: &Press) {
            let view = self.view.upgrade().unwrap();
            let text_field = TextField::from_view(view.clone());
            let text_field_behavior = text_field.behavior();
            let key = press.key();

            if key.key_code() == KeyCode::LAlt || key.key_code() == KeyCode::RAlt {
                let count = text_field_behavior.holding_alternative.get();
                text_field_behavior.holding_alternative.set(count - 1);
            }

            if key.key_code() == KeyCode::LShift || key.key_code() == KeyCode::RShift {
                let count = text_field_behavior.holding_shift.get();
                text_field_behavior.holding_shift.set(count - 1);
            }
        }

        fn press_began(&self, press: &Press) {
            let view = self.view.upgrade().unwrap();
            let text_field = TextField::from_view(view.clone());
            let label = text_field.label();
            let text_field_behavior = text_field.behavior();

            let mut carats = text_field_behavior.carats.borrow_mut();

            let key = press.key();

            if key.key_code() == KeyCode::LAlt || key.key_code() == KeyCode::RAlt {
                let count = text_field_behavior.holding_alternative.get();
                text_field_behavior.holding_alternative.set(count + 1);
            }

            if key.key_code() == KeyCode::LShift || key.key_code() == KeyCode::RShift {
                let count = text_field_behavior.holding_shift.get();
                text_field_behavior.holding_shift.set(count + 1);
            }

            match key.key_code() {
                KeyCode::Left => {
                    let mut distance = 1;
                    if key.modifier_flags().contains(&ModifierFlag::Alternate) {
                        distance = 2;
                    }

                    let highlight = key.modifier_flags().contains(&ModifierFlag::Shift);

                    for carat in carats.iter_mut() {
                        let index = carat.character_index.get();

                        let mut new_index = index as i32 - distance;
                        if new_index < 0 {
                            new_index = 0;
                        }
                        let new_index = new_index as usize;
                        carat.character_index.set(new_index as usize);
                        if highlight {
                            let mut rhs_select = index;
                            if let Some(existing_selection) = &carat.selection {
                                rhs_select = existing_selection.end;
                            }
                            text_field.select_range(carat, new_index..rhs_select);
                        } else {
                            text_field.select_range(carat, 0..0);
                            }

                        if let Some(carat_view) = carat.view.upgrade() {
                            carat_view.set_hidden(false);
                            carat_view.set_needs_display();
                        }
                        carat.delay_animation.set(true);
                    }
                },
                KeyCode::Right => {
                    let mut distance = 1;
                    if key.modifier_flags().contains(&ModifierFlag::Alternate) {
                        distance = 2;
                    }

                    let highlight = key.modifier_flags().contains(&ModifierFlag::Shift);

                    for carat in carats.iter_mut() {
                        let index = carat.character_index.get();

                        let mut new_index = index + distance;
                        if new_index > label.text().len() {
                            new_index = label.text().len();
                        }
                        carat.character_index.set(new_index as usize);
                        if highlight {
                            let mut lhs_select = index;
                            if let Some(existing_selection) = &carat.selection {
                                lhs_select = existing_selection.start;
                            }
                            text_field.select_range(carat, lhs_select..new_index);
                        } else {
                            text_field.select_range(carat, 0..0);
                        }

                        if let Some(carat_view) = carat.view.upgrade() {
                            carat_view.set_hidden(false);
                            carat_view.set_needs_display();
                        }
                        carat.delay_animation.set(true);
                    }
                },
                KeyCode::Backspace => {
                    let mut distance = 1;
                    if key.modifier_flags().contains(&ModifierFlag::Alternate) {
                        distance = 2;
                    }
                    for carat in carats.iter() {
                        let index = carat.character_index.get();
                        if index > 0 {
                            let mut new_index = index as i32 - distance;
                            if new_index < 0 {
                                new_index = 0;
                            }
                            let new_index = new_index as usize;
                            carat.character_index.set(new_index);
                            label.replace_text_in_range(new_index..index, "");
                        }
                        if let Some(carat_view) = carat.view.upgrade() {
                            carat_view.set_hidden(false);
                            carat_view.set_needs_display();
                        }
                        carat.delay_animation.set(true);
                    }
                },
                _ => ()
            }
        }
    }
);

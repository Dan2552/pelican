use crate::graphics::{Rectangle, Size, Point};
use crate::ui::view::{View, WeakView};
use crate::ui::view::DefaultBehavior;
use crate::ui::Color;
use crate::macros::*;
use crate::ui::view::Label;
use crate::ui::run_loop::RunLoop;
use crate::ui::timer::Timer;
use crate::ui::touch::Touch;
use crate::ui::press::Press;
use crate::ui::key::{KeyCode, ModifierFlag};
use std::cell::RefCell;
use std::time::Duration;
use std::cell::Cell;
use std::ops::Range;
use crate::text::word_boundary;
use std::collections::HashMap;
use std::time::Instant;
use crate::text::text::Text;
use crate::platform::clipboard;

pub(crate) struct Carat {
    view: WeakView,
    character_index: Cell<usize>,
    selection: Option<Selection>
}

impl Drop for Carat {
    fn drop(&mut self) {
        if let Some(view) = self.view.upgrade() {
            view.remove_from_superview();
        }
    }
}

pub(crate) struct Selection {
    start: usize,
    end: usize,
    views: RefCell<Vec<WeakView>>
}

impl Drop for Selection {
    fn drop(&mut self) {
        for view in self.views.borrow().iter() {
            if let Some(view) = view.upgrade() {
                view.remove_from_superview();
            }
        }
    }
}

impl Drop for TextFieldBehavior {
    fn drop(&mut self) {
        if let Some(timer) = &self.carat_animation_timer {
            timer.invalidate();
        }
    }
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
        touch_began_at_index: Cell<usize>,

        // A timer responsible for animating the carats.
        carat_animation_timer: Option<Timer>,

        delay_animation: Cell<bool>,

        last_click: Cell<Instant>,
        click_count: Cell<u8>
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
                Cell::new(0),
                None,
                Cell::new(false),
                Cell::new(Instant::now()),
                Cell::new(0)
            );

            text_field.view.add_subview(label.view);
            text_field.spawn_carat(0);

            let weak_text_field = text_field.view.downgrade();
            let carat_animation_timer = Timer::new_repeating(Duration::from_millis(500), move || {
                if let Some(view) = weak_text_field.upgrade() {
                    let text_field = TextField::from_view(view);
                    text_field.animate_carats();
                }
            });
            let run_loop = RunLoop::borrow();
            run_loop.add_timer(carat_animation_timer);

            text_field
        }

        pub fn label(&self) -> Label {
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

            rendering.character_at_position(position).unwrap_or(label.text_len())
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
                selection: None
            };

            carats.push(carat);

            self.view.set_needs_display();
        }

        fn animate_carats(&self) {
            let behavior = self.behavior();

            let mut hidden: Option<bool> = None;
            for carat in behavior.carats.borrow().iter() {
                if let Some(view) = carat.view.upgrade() {
                    if behavior.delay_animation.get() {
                        view.set_hidden(false);
                    } else {
                        if hidden.is_none() {
                            hidden = Some(!view.is_hidden());
                        }
                        view.set_hidden(hidden.unwrap());
                    }
                }
            }

            if behavior.delay_animation.get() {
                behavior.delay_animation.set(false);
            }
        }

        fn select_all(&self) {
            self.remove_carats();
            let behavior = self.behavior();
            let text_len = self.label().text_len();
            self.spawn_carat(text_len);
            let mut carats = behavior.carats.borrow_mut();
            self.select(carats.last_mut().unwrap(), 0, text_len);
        }

        pub(crate) fn insert_str(&self, index: usize, text: &str) {
            // TODO
        }

        pub(crate) fn replace_range(&self, range: Range<usize>, text: &str) {
            // TODO
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

        /// Returns the selected text, if any. Because there are multiple
        /// carats, the text is returned as a `Vec<String>`.
        fn selected_text(&self) -> Vec<String> {
            let mut result = Vec::new();
            let behavior = self.behavior();

            let label = self.label();
            let text = label.text();

            for carat in behavior.carats.borrow().iter() {
                if let Some(selection) = carat.selection.as_ref() {
                    let selected_text = &text[selection.start..selection.end];
                    result.push(String::from(selected_text));
                }
            }

            result
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

        /// If more than one cursor is in the same spot, only one should
        /// survive.
        fn consume_and_sort_cursors(&self) {
            let behavior = self.behavior();
            let mut carats = behavior.carats.borrow_mut();

            let mut indexes = HashMap::new();
            carats.retain(|carat| {
                let character_index = carat.character_index.get();
                if indexes.contains_key(&character_index) {
                    return false;
                } else {
                    indexes.insert(character_index, true);
                    return true;
                }
            });

            carats.sort_by(|a, b| a.character_index.get().cmp(&b.character_index.get()));
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
            } else if self.holding_alternative.get() > 0 {
                text_field.spawn_carat(touched_character_index);
                return;
            }

            text_field.remove_carats();
            text_field.spawn_carat(touched_character_index);


            if self.last_click.get().elapsed().as_millis() < 500 {
                self.click_count.set(self.click_count.get() + 1);

                let label = text_field.label();
                let text = label.text();

                if self.click_count.get() == 2 {
                    // go forward, back, and then forward again incase it
                    // started on whitespace (otherwise multiple words would
                    // be selected).
                    let rhs = word_boundary::find_word_boundary(text, touched_character_index, true);
                    let lhs = word_boundary::find_word_boundary(text, rhs, false);
                    let rhs = word_boundary::find_word_boundary(text, touched_character_index, true);

                    let mut carats = self.carats.borrow_mut();
                    let carat = carats.first_mut().unwrap();
                    text_field.select(carat, lhs, rhs);
                    carat.character_index.set(rhs);
                } else if self.click_count.get() == 3 {
                    // go forward, back, and then forward again incase it
                    // started on whitespace (otherwise multiple words would
                    // be selected).
                    let rhs = word_boundary::find_line_boundary(text, touched_character_index, true);
                    let lhs = word_boundary::find_line_boundary(text, rhs, false);
                    let rhs = word_boundary::find_line_boundary(text, touched_character_index, true);

                    let mut carats = self.carats.borrow_mut();
                    let carat = carats.first_mut().unwrap();
                    text_field.select(carat, lhs, rhs);
                    carat.character_index.set(rhs);
                }

            } else {
                self.click_count.set(1);
            }

            self.last_click.set(Instant::now());
        }

        fn text_input_did_receive(&self, text: &str) {
            let view = self.view.upgrade().unwrap();
            let text_field = TextField::from_view(view.clone());
            text_field.consume_and_sort_cursors();
            let label = text_field.label();
            let text_field_behavior = text_field.behavior();

            let mut carats = text_field_behavior.carats.borrow_mut();

            let mut extra_movement_for_following_carat: i32 = 0;

            for carat in carats.iter_mut() {
                // Adjust for extra_movement_for_following_carat
                {
                    let index = carat.character_index.get();
                    let mut new_index = index as i32 + extra_movement_for_following_carat;
                    if new_index < 0 {
                        new_index = 0;
                    }
                    if new_index > label.text_len() as i32 {
                        new_index = label.text_len() as i32;
                    }
                    carat.character_index.set(new_index as usize);

                    if carat.selection.is_some() {
                        let selection_start = carat.selection.as_ref().unwrap().start as i32 + extra_movement_for_following_carat;
                        let selection_end = carat.selection.as_ref().unwrap().end as i32 + extra_movement_for_following_carat;

                        text_field.select(carat, selection_start as usize, selection_end as usize);
                    }
                }

                if let Some(selection) = &carat.selection {
                    label.replace_text_in_range(selection.start..selection.end, &text);
                    extra_movement_for_following_carat -= (selection.end - selection.start) as i32;
                    carat.character_index.set(selection.start + text.len());
                } else {
                    let index = carat.character_index.get();
                    label.insert_text_at_index(index, text);
                    carat.character_index.set(index + Text::from(text).len());
                }

                carat.selection = None;

                extra_movement_for_following_carat += text.len() as i32;

                if let Some(carat_view) = carat.view.upgrade() {
                    carat_view.set_hidden(false);
                }
            }

            self.delay_animation.set(true);
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
            text_field.consume_and_sort_cursors();
            let label = text_field.label();
            let text_field_behavior = text_field.behavior();

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
                KeyCode::C => {
                    if key.modifier_flags().contains(&ModifierFlag::Control) || key.modifier_flags().contains(&ModifierFlag::Command) {
                        let text_to_copy = text_field.selected_text().join("\n");
                        clipboard::set_string(&text_to_copy);
                    }
                },
                KeyCode::V => {
                    if key.modifier_flags().contains(&ModifierFlag::Control) || key.modifier_flags().contains(&ModifierFlag::Command) {
                        if let Some(text_to_paste) = clipboard::get_string() {
                            self.text_input_did_receive(&text_to_paste);
                        }
                    }
                },
                KeyCode::Left => {
                    let highlight = key.modifier_flags().contains(&ModifierFlag::Shift);
                    let mut carats = text_field_behavior.carats.borrow_mut();
                    for carat in carats.iter_mut() {
                        let index = carat.character_index.get();

                        let mut distance = 1;
                        if key.modifier_flags().contains(&ModifierFlag::Alternate) || key.modifier_flags().contains(&ModifierFlag::Command) {
                            let text_field = TextField::from_view(self.view.upgrade().unwrap());
                            let label = text_field.label();
                            let text = label.text();
                            let boundary: usize;
                            if key.modifier_flags().contains(&ModifierFlag::Alternate) {
                                boundary = word_boundary::find_word_boundary(text, index, false);
                            } else {
                                boundary = word_boundary::find_line_boundary(text, index, false);
                            }

                            distance = index as i32 - boundary as i32;
                        }

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
                        self.delay_animation.set(true);
                    }
                },
                KeyCode::Right => {
                    let highlight = key.modifier_flags().contains(&ModifierFlag::Shift);
                    let mut carats = text_field_behavior.carats.borrow_mut();
                    for carat in carats.iter_mut() {
                        let index = carat.character_index.get();

                        let mut distance = 1;
                        if key.modifier_flags().contains(&ModifierFlag::Alternate) || key.modifier_flags().contains(&ModifierFlag::Command) {
                            let text_field = TextField::from_view(self.view.upgrade().unwrap());
                            let label = text_field.label();
                            let text = label.text();
                            let boundary: usize;
                            if key.modifier_flags().contains(&ModifierFlag::Alternate) {
                                boundary = word_boundary::find_word_boundary(text, index, true);
                            } else {
                                boundary = word_boundary::find_line_boundary(text, index, true);
                            }

                            distance = boundary as i32 - index as i32;
                        }

                        let mut new_index = index as i32 + distance;
                        if new_index > label.text_len() as i32 {
                            new_index = label.text_len() as i32;
                        }
                        carat.character_index.set(new_index as usize);
                        if highlight {
                            let mut lhs_select = index;
                            if let Some(existing_selection) = &carat.selection {
                                lhs_select = existing_selection.start;
                            }
                            let new_index = new_index as usize;
                            text_field.select_range(carat, lhs_select..new_index);
                        } else {
                            text_field.select_range(carat, 0..0);
                        }

                        if let Some(carat_view) = carat.view.upgrade() {
                            carat_view.set_hidden(false);
                            carat_view.set_needs_display();
                        }
                        self.delay_animation.set(true);
                    }
                },
                KeyCode::Backspace => {
                    let mut extra_movement_for_following_carat: i32 = 0;
                    let mut carats = text_field_behavior.carats.borrow_mut();
                    for carat in carats.iter_mut() {
                        // First move the cursor if other cursors have caused
                        // this one to move.
                        {
                            let index = carat.character_index.get();
                            let mut new_index: i32 = index as i32 - extra_movement_for_following_carat;
                            if new_index < 0 {
                                new_index = 0;
                            }
                            carat.character_index.set(new_index as usize);

                            if let Some(selection) = &carat.selection {
                                let start = selection.start as i32 - extra_movement_for_following_carat;
                                let end = selection.end as i32 - extra_movement_for_following_carat;
                                let start = start as usize;
                                let end = end as usize;
                                text_field.select_range(carat, start..end);
                            }
                        }

                        // And then move again for this deletion
                        {
                            let mut distance = 1;
                            if let Some(selection) = &carat.selection {
                                distance = selection.end as i32 - selection.start as i32;
                                if carat.character_index.get() != selection.end {
                                    carat.character_index.set(selection.end);
                                }
                                text_field.select_range(carat, 0..0);
                            } else if key.modifier_flags().contains(&ModifierFlag::Alternate) || key.modifier_flags().contains(&ModifierFlag::Command) {
                                let index = carat.character_index.get();
                                let text_field = TextField::from_view(self.view.upgrade().unwrap());
                                let label = text_field.label();
                                let text = label.text();
                                let boundary: usize;
                                if key.modifier_flags().contains(&ModifierFlag::Alternate) {
                                    boundary = word_boundary::find_word_boundary(text, index, false);
                                } else {
                                    boundary = word_boundary::find_line_boundary(text, index, false);
                                }

                                distance = index as i32 - boundary as i32;
                            }

                            let index = carat.character_index.get();
                            let mut new_index = index as i32 - distance;
                            if new_index < 0 {
                                new_index = 0;
                            }
                            let new_index = new_index as usize;
                            carat.character_index.set(new_index as usize);

                            label.replace_text_in_range(new_index..index, "");
                            let distance = index as i32 - new_index as i32;
                            extra_movement_for_following_carat += distance;
                        }

                        if let Some(carat_view) = carat.view.upgrade() {
                            carat_view.set_hidden(false);
                            carat_view.set_needs_display();
                        }
                        self.delay_animation.set(true);
                    }
                },
                KeyCode::A => {
                    if key.modifier_flags().contains(&ModifierFlag::Command) {
                        text_field.select_all();
                    }
                },
                _ => ()
            }
        }
    }
);

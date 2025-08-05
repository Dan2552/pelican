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
use std::time::Instant;
use crate::text::text::Text;
use crate::platform::clipboard;
use crate::ui::history::text_field::text_insertion::TextInsertion;
use crate::ui::history::text_field::text_backspace::TextBackspace;
use crate::platform::history::Action;
use crate::platform::history::History;
use crate::ui::history::text_field::carat_snapshot::CaratSnapshot;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum CursorMovement {
    Character,
    Word,
    Line
}

pub(crate) struct Carat {
    view: WeakView,
    character_index: Cell<usize>,
    selection: Option<Selection>
}

impl Carat {
    fn snapshot(&self) -> CaratSnapshot {
        let selection_snapshot = match &self.selection {
            Some(selection) => Some(selection.start..selection.end),
            None => None
        };

        CaratSnapshot::new(self.character_index.get(), selection_snapshot)
    }
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
        let mut carat_animation_timer = self.carat_animation_timer.borrow_mut();

        if let Some(timer) = carat_animation_timer.as_mut() {
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
        carat_animation_timer: RefCell<Option<Timer>>,

        delay_animation: Cell<bool>,

        last_click: Cell<Instant>,
        click_count: Cell<u8>,

        history: RefCell<History>,

        text_change: RefCell<Option<Box<dyn Fn(&TextField) -> ()>>>
    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>, text: String) -> TextField {
            let label_padding = 8;
            let label_frame = Rectangle::new(
                label_padding as i32,
                label_padding as i32,
                frame.width() - (label_padding * 2),
                frame.height() - (label_padding * 2)
            );
            let label = Label::new(label_frame, text);
            label.view.set_tag(1);
            label.view.set_user_interaction_enabled(false);

            let carats = RefCell::new(Vec::new());
            let text_field = TextField::new_all(
                frame,
                carats,
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
                RefCell::new(None),
                Cell::new(false),
                Cell::new(Instant::now()),
                Cell::new(0),
                RefCell::new(History::new()),
                RefCell::new(None)
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
            run_loop.add_timer(carat_animation_timer.clone());

            let behavior = text_field.behavior();
            behavior.carat_animation_timer.replace(Some(carat_animation_timer));

            text_field.clone()
        }

        pub fn on_text_change(&self, action: impl Fn(&TextField) -> () + 'static) {
            let behavior = self.behavior();
            behavior.text_change.replace(Some(Box::new(action)));
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
            let render_scale = rendering.render_scale();

            let position = Point {
                x: (position.x as f32 * render_scale).round() as i32,
                y: (position.y as f32 * render_scale).round() as i32
            };

            rendering.character_at_position(position)
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

            self.select_range(carat, &(lhs..rhs));
        }

        fn select_range(&self, carat: &mut Carat, range: &Range<usize>) {
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
            {
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
            }
            self.consume_and_sort_cursors();
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

        pub(crate) fn restore_carat_snapshots(&self, snapshot: &Vec<CaratSnapshot>) {
            self.remove_carats();

            let behavior = self.behavior();

            for carat_snapshot in snapshot.iter() {
                self.spawn_carat(carat_snapshot.character_index());
                let mut carats = behavior.carats.borrow_mut();
                let carat = carats.last_mut().unwrap();
                if let Some(selection) = carat_snapshot.selection() {
                    self.select_range(carat, selection);
                }
            }
        }

        pub(crate) fn carat_snapshots(&self) -> Vec<CaratSnapshot> {
            let behavior = self.behavior();
            let carats = behavior.carats.borrow();
            carats.iter().map(|carat| carat.snapshot()).collect()
        }

        /// Returns the indexes of the carats.
        pub(crate) fn carat_indexes(&self) -> Vec<usize> {
            let behavior = self.behavior();
            let carats = behavior.carats.borrow();
            carats.iter().map(|carat| carat.character_index.get()).collect()
        }

        /// Multi-carat operation.
        ///
        /// Deletes a given amount of characters at each of the current carat
        /// positions. This will move the carats to the beginning of the deleted
        /// characters.
        pub(crate) fn backspace(&self, movement: CursorMovement, amount: usize) -> Vec<String> {
            let mut deleted_text = Vec::new();

            let label = self.label();
            let behavior = self.behavior();
            let mut extra_movement_for_following_carat: i32 = 0;
            let mut carats = behavior.carats.borrow_mut();

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
                        self.select_range(carat, &(start..end));
                    }
                }

                // And then move again for this deletion
                {
                    let mut distance = amount as i32;
                    if let Some(selection) = &carat.selection {
                        distance = selection.end as i32 - selection.start as i32;
                        if carat.character_index.get() != selection.end {
                            carat.character_index.set(selection.end);
                        }
                        self.select_range(carat, &(0..0));
                    } else if movement != CursorMovement::Character {
                        let index = carat.character_index.get();
                        let text = label.text();
                        let boundary: usize;
                        if movement == CursorMovement::Word {
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

                    deleted_text.push(label.text()[new_index..index].to_string());
                    label.replace_text_in_range(new_index..index, "");
                    let distance = index as i32 - new_index as i32;
                    extra_movement_for_following_carat += distance;
                }

                if let Some(carat_view) = carat.view.upgrade() {
                    carat_view.set_hidden(false);
                    carat_view.set_needs_display();
                }
                behavior.delay_animation.set(true);
            }

            deleted_text
        }

        /// Multi-carat operation.
        ///
        /// Inserts a str at each of the current carat positions. This will move
        /// the carats to the end of the inserted text.
        ///
        /// If any of the carats are currently selected, the inserted text will
        /// replace the selected text.
        ///
        /// Returns the contents of any text that was replaced (one element
        /// per carat).
        pub(crate) fn insert_str(&self, text: &str) -> Vec<Option<String>> {
            let mut result = Vec::new();

            let view = &self.view;
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
                    result.push(Some(label.text()[selection.start..selection.end].to_string()));
                    label.replace_text_in_range(selection.start..selection.end, &text);
                    extra_movement_for_following_carat -= (selection.end - selection.start) as i32;
                    carat.character_index.set(selection.start + text.len());
                } else {
                    result.push(None);
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

            let behavior = self.behavior();
            behavior.delay_animation.set(true);

            result
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

        fn carat_positions(&self) -> Vec<usize> {
            let behavior = self.behavior();
            let carats = behavior.carats.borrow();
            carats.iter().map(|carat| carat.character_index.get()).collect()
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

        /// Called internally by `press_began` (when holding shift and pressing
        /// a directional key) and `touches_moved`.
        fn move_carat_selecting(&self, carat: &mut Carat, previous_character_index: usize, target_character_index: usize) {
            let text_field = self;

            if let Some(selection) = carat.selection.as_mut() {
                let current_cursor = carat.character_index.get();
                carat.character_index.set(target_character_index);

                let start_distance = (current_cursor as i32  - selection.start as i32).abs();
                let end_distance = (current_cursor as i32 - selection.end as i32).abs();
                if start_distance < end_distance {
                    selection.start = target_character_index;
                    text_field.position_selection(&carat.selection.as_ref().unwrap());
                } else if end_distance < start_distance {
                    selection.end = target_character_index;
                    text_field.position_selection(&carat.selection.as_ref().unwrap());
                } else {
                    let start = previous_character_index;

                    text_field.select(carat, start, target_character_index);
                    carat.character_index.set(target_character_index);
                }

            } else {
                let start = previous_character_index;

                text_field.select(carat, start, target_character_index);
                carat.character_index.set(target_character_index);
            }
        }

        fn position_selection(&self, selection: &Selection) {
            let label = self.label();
            let label_behavior = label.behavior();
            let rendering = label_behavior.rendering();

            let render_scale = rendering.render_scale();

            for view in selection.views.borrow().iter() {
                if let Some(view) = view.upgrade() {
                    view.remove_from_superview();
                }
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

            // if we sort upfront, then any overlapping cursors / selections
            // should be next to each other.
            carats.sort_by(|a, b| a.character_index.get().cmp(&b.character_index.get()));

            // 1. search for intersecting, set the character index to be the same
            for index in 0..(carats.len()) {
                if index == 0 {
                    continue;
                }

                let mut last_snapshot = carats[index - 1].snapshot();
                let current_snapshot = carats[index].snapshot();

                if current_snapshot.selection_intersects(&last_snapshot) {
                    // Replace the previous carat's selection with one that
                    // encompasses both.
                    {
                        let last_carat = carats.get_mut(index - 1).unwrap();
                        let merged_end = current_snapshot.selection().as_ref().unwrap().end;
                        last_carat.selection = Some(Selection {
                            start: last_snapshot.selection().as_ref().unwrap().start,
                            end: merged_end,
                            views: RefCell::new(Vec::new())
                        });

                        // If the cursor wasn't on the left hand side of the
                        // selection, then we need to move it to the new right.
                        if last_snapshot.character_index() != last_snapshot.selection().as_ref().unwrap().start {
                            last_carat.character_index.replace(merged_end);
                            last_snapshot = last_carat.snapshot();
                        }

                        self.position_selection(&last_carat.selection.as_ref().unwrap());
                    }

                    // Set the current carat to matching character index, so it
                    // will be removed.
                    {
                        let current_carat = carats.get_mut(index).unwrap();
                        current_carat.character_index.set(last_snapshot.character_index());
                    }
                }
            }

            // 2. drain ones that are the same
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
                let touched_character_index = text_field.touch_to_index(touches.first().unwrap());

                let previous_character_index = self.touch_began_at_index.get();
                text_field.move_carat_selecting(last_cursor, previous_character_index, touched_character_index);
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

            let mut text_insertion = TextInsertion::new(
                self.view.clone(),
                text.to_string(),
                text_field.carat_snapshots()
            );

            text_insertion.forward();

            let mut history = self.history.borrow_mut();
            history.add(Box::new(text_insertion));

            if let Some(text_change) = self.text_change.borrow().as_ref() {
                text_change(&text_field);
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
                KeyCode::X => {
                    if key.modifier_flags().contains(&ModifierFlag::Control) || key.modifier_flags().contains(&ModifierFlag::Command) {
                        let text_to_copy = text_field.selected_text().join("\n");
                        clipboard::set_string(&text_to_copy);

                        let mut text_backspace = TextBackspace::new(
                            self.view.clone(),
                            1,
                            CursorMovement::Character,
                            text_field.carat_snapshots()
                        );

                        text_backspace.forward();

                        let mut history = self.history.borrow_mut();
                        history.add(Box::new(text_backspace));
                    }
                },
                KeyCode::Z => {
                    if key.modifier_flags().contains(&ModifierFlag::Control) || key.modifier_flags().contains(&ModifierFlag::Command) {
                        let mut history = self.history.borrow_mut();
                        if key.modifier_flags().contains(&ModifierFlag::Shift) {
                            history.redo();
                        } else {
                            history.undo();
                        }

                        if let Some(text_change) = self.text_change.borrow().as_ref() {
                            text_change(&text_field);
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

                        if highlight {
                            let previous_character_index = carat.character_index.get();
                            text_field.move_carat_selecting(carat, previous_character_index, new_index as usize);
                        } else {
                            carat.character_index.set(new_index as usize);
                            text_field.select_range(carat, &(0..0));
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

                        if highlight {
                            let previous_character_index = carat.character_index.get();
                            text_field.move_carat_selecting(carat, previous_character_index, new_index as usize);
                        } else {
                            carat.character_index.set(new_index as usize);
                            text_field.select_range(carat, &(0..0));
                        }

                        if let Some(carat_view) = carat.view.upgrade() {
                            carat_view.set_hidden(false);
                            carat_view.set_needs_display();
                        }
                        self.delay_animation.set(true);
                    }
                },
                KeyCode::Up => {
                    let highlight = key.modifier_flags().contains(&ModifierFlag::Shift);
                    let mut carats = text_field_behavior.carats.borrow_mut();
                    for carat in carats.iter_mut() {
                        let new_index;
                        let label_behavior = label.behavior();
                        let rendering = label_behavior.rendering();
                        let position = rendering.position_for_character_at_index(carat.character_index.get());

                        if position.y < rendering.line_height_for_character_at_index(0) as i32 {
                            new_index = 0;
                        } else {
                            let line_height = rendering.line_height_for_character_at_index(carat.character_index.get());
                            let new_position = Point {
                                x: position.x,
                                y: position.y - (line_height / 2) as i32,
                            };

                            new_index = rendering.character_at_position(new_position);
                        }

                        if highlight {
                            let previous_character_index = carat.character_index.get();
                            text_field.move_carat_selecting(carat, previous_character_index, new_index as usize);
                        } else {
                            carat.character_index.set(new_index as usize);
                            text_field.select_range(carat, &(0..0));
                        }

                        if let Some(carat_view) = carat.view.upgrade() {
                            carat_view.set_needs_display();
                        }
                        self.delay_animation.set(true);
                    }
                },
                KeyCode::Down => {
                    let highlight = key.modifier_flags().contains(&ModifierFlag::Shift);
                    let mut carats = text_field_behavior.carats.borrow_mut();
                    for carat in carats.iter_mut() {
                        let label_behavior = label.behavior();
                        let rendering = label_behavior.rendering();
                        let position = rendering.position_for_character_at_index(carat.character_index.get());

                        let last_index = label.text().len();
                        let last_line_height = rendering.line_height_for_character_at_index(last_index as usize);
                        let last_character_position = rendering.position_for_character_at_index(last_index as usize);
                        let bottom = last_character_position.y + last_line_height as i32;

                        let line_height = rendering.line_height_for_character_at_index(carat.character_index.get());

                        let new_position = Point {
                            x: position.x,
                            y: position.y + line_height as i32 + (line_height / 2) as i32,
                        };

                        let new_index;
                        if new_position.y >= bottom {
                            new_index = last_index;
                        } else {
                            new_index = rendering.character_at_position(new_position);
                        }

                        if highlight {
                            text_field.move_carat_selecting(carat, carat.character_index.get(), new_index);
                        } else {
                            carat.character_index.set(new_index);
                            text_field.select_range(carat, &(0..0));
                        }
                        if let Some(carat_view) = carat.view.upgrade() {
                            carat_view.set_needs_display();
                        }
                        self.delay_animation.set(true);
                    }
                },
                KeyCode::Backspace => {
                    let view = self.view.upgrade().unwrap();
                    let text_field = TextField::from_view(view);

                    let mut movement_type = CursorMovement::Character;

                    if key.modifier_flags().contains(&ModifierFlag::Alternate) {
                        movement_type = CursorMovement::Word;
                    } else if key.modifier_flags().contains(&ModifierFlag::Command) {
                        movement_type = CursorMovement::Line;
                    }

                    let mut text_backspace = TextBackspace::new(
                        self.view.clone(),
                        1,
                        movement_type,
                        text_field.carat_snapshots()
                    );

                    text_backspace.forward();

                    let mut history = self.history.borrow_mut();
                    history.add(Box::new(text_backspace));

                    if let Some(text_change) = self.text_change.borrow().as_ref() {
                        text_change(&text_field);
                    }
                },
                KeyCode::A => {
                    if key.modifier_flags().contains(&ModifierFlag::Command) {
                        text_field.select_all();
                    }
                },
                KeyCode::Return => {
                    let view = self.view.upgrade().unwrap();
                    let text_field = TextField::from_view(view.clone());

                    let mut text_insertion = TextInsertion::new(
                        self.view.clone(),
                        '\n'.to_string(),
                        text_field.carat_snapshots()
                    );

                    text_insertion.forward();

                    let mut history = self.history.borrow_mut();
                    history.add(Box::new(text_insertion));

                    if let Some(text_change) = self.text_change.borrow().as_ref() {
                        text_change(&text_field);
                    }
                }
                _ => ()
            }

            text_field.consume_and_sort_cursors();
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::view::behavior::Behavior;
    use crate::ui::key::{Key, ModifierFlag};
    use crate::ui::press::Press;

    #[test]
    fn test_text_field_integration() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());
        let behavior = text_field.behavior();

        behavior.text_input_did_receive("hello");

        assert_eq!(text_field.label().text().string(), "hello");

        let key = Key::new(KeyCode::Z, vec![ModifierFlag::Command]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(text_field.label().text().string(), "");

        let key = Key::new(KeyCode::Z, vec![ModifierFlag::Command, ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(text_field.label().text().string(), "hello");

        behavior.text_input_did_receive("hello");

        let key = Key::new(KeyCode::Left, vec![ModifierFlag::Shift]);
        let press = Press::new(key);

        for _ in 0..4 {
            behavior.press_began(&press);
            behavior.press_ended(&press);
        }

        behavior.text_input_did_receive("i");

        assert_eq!(text_field.label().text().string(), "hellohi");

        let key = Key::new(KeyCode::A, vec![ModifierFlag::Command]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let key = Key::new(KeyCode::Backspace, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(text_field.label().text().string(), "");

        behavior.text_input_did_receive("hello");
        assert_eq!(text_field.carat_positions().len(), 1);
        assert_eq!(text_field.carat_positions()[0], 5);

        text_field.spawn_carat(0);
        assert_eq!(text_field.carat_positions().len(), 2);
        assert_eq!(text_field.carat_positions()[0], 0);
        assert_eq!(text_field.carat_positions()[1], 5);

        behavior.text_input_did_receive("world");

        assert_eq!(text_field.label().text().string(), "worldhelloworld");

        let key = Key::new(KeyCode::Right, vec![ModifierFlag::Command]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(text_field.carat_positions()[0], 15);

        let key = Key::new(KeyCode::Left, vec![ModifierFlag::Shift, ModifierFlag::Alternate]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        behavior.text_input_did_receive("hello world");

        assert_eq!(text_field.carat_positions().len(), 1);
        assert_eq!(text_field.carat_positions()[0], 11);
        assert_eq!(text_field.label().text().string(), "hello world");

        let key = Key::new(KeyCode::Left, vec![ModifierFlag::Alternate]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        behavior.text_input_did_receive("wide ");
        assert_eq!(text_field.label().text().string(), "hello wide world");

        let key = Key::new(KeyCode::A, vec![ModifierFlag::Command]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let key = Key::new(KeyCode::Backspace, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(text_field.label().text().string(), "");

        behavior.text_input_did_receive("hello");
        assert_eq!(text_field.carat_positions().len(), 1);
        assert_eq!(text_field.carat_positions()[0], 5);

        text_field.spawn_carat(0);
        text_field.spawn_carat(1);

        assert_eq!(text_field.carat_positions().len(), 3);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 3);
        assert_eq!(cursors[0].character_index(), 0);
        assert_eq!(cursors[0].selection(), &None);
        assert_eq!(cursors[1].character_index(), 1);
        assert_eq!(cursors[1].selection(), &None);
        assert_eq!(cursors[2].character_index(), 5);
        assert_eq!(cursors[2].selection(), &None);

        let key = Key::new(KeyCode::Right, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 3);
        assert_eq!(cursors[0].character_index(), 1);
        assert_eq!(cursors[0].selection(), &Some(0..1));
        assert_eq!(cursors[1].character_index(), 2);
        assert_eq!(cursors[1].selection(), &Some(1..2));
        assert_eq!(cursors[2].character_index(), 5);
        assert_eq!(cursors[2].selection(), &None);

        let key = Key::new(KeyCode::Right, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 2);
        assert_eq!(cursors[0].character_index(), 3);
        assert_eq!(cursors[0].selection(), &Some(0..3));
        assert_eq!(cursors[1].character_index(), 5);
        assert_eq!(cursors[1].selection(), &None);

        let key = Key::new(KeyCode::A, vec![ModifierFlag::Command]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let key = Key::new(KeyCode::Backspace, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        behavior.text_input_did_receive("hi");

        assert_eq!(text_field.carat_positions().len(), 1);
        assert_eq!(text_field.carat_positions()[0], 2);
        assert_eq!(text_field.label().text().string(), "hi");

        let key = Key::new(KeyCode::Left, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let key = Key::new(KeyCode::Left, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let key = Key::new(KeyCode::Right, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 1);
        assert_eq!(cursors[0].selection(), &Some(1..2));

        let key = Key::new(KeyCode::Left, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let key = Key::new(KeyCode::Right, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let key = Key::new(KeyCode::Right, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let key = Key::new(KeyCode::Left, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(text_field.label().text().string(), "hi");

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 1);
        assert_eq!(cursors[0].selection(), &Some(0..1));

        let key = Key::new(KeyCode::Down, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        // it should move to the end of the line
        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 2);
        assert_eq!(cursors[0].selection(), &None);

        let key = Key::new(KeyCode::Up, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        // it should move to the start of the line
        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 0);
        assert_eq!(cursors[0].selection(), &None);

        behavior.text_input_did_receive("hi\n");

        assert_eq!(text_field.label().text().string(), "hi\nhi");

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 3);
        assert_eq!(cursors[0].selection(), &None);

        let key = Key::new(KeyCode::Up, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 0);
        assert_eq!(cursors[0].selection(), &None);

        let key = Key::new(KeyCode::Down, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 3);
        assert_eq!(cursors[0].selection(), &Some(0..3));

        let key = Key::new(KeyCode::Down, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 5);
        assert_eq!(cursors[0].selection(), &Some(0..5));

        let key = Key::new(KeyCode::Up, vec![ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 2);
        assert_eq!(cursors[0].selection(), &Some(0..2));
    }

    #[test]
    fn test_text_field_up_down_through_multiple_lines() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());
        let behavior = text_field.behavior();

        behavior.text_input_did_receive("a\n\nb");

        assert_eq!(text_field.label().text().string(), "a\n\nb");

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 4);
        assert_eq!(cursors[0].selection(), &None);

        let key = Key::new(KeyCode::Up, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 2);
        assert_eq!(cursors[0].selection(), &None);

        let key = Key::new(KeyCode::Up, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 0);
        assert_eq!(cursors[0].selection(), &None);

        let key = Key::new(KeyCode::Down, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 2);
        assert_eq!(cursors[0].selection(), &None);

        let key = Key::new(KeyCode::Down, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        let cursors = text_field.carat_snapshots();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].character_index(), 3);
        assert_eq!(cursors[0].selection(), &None);
    }

    #[test]
    fn test_on_text_change() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());
        let behavior = text_field.behavior();

        let test = std::rc::Rc::new(RefCell::new("".to_string()));

        let test_clone = test.clone();
        text_field.on_text_change(move |text_field| {
            test_clone.replace(text_field.label().text().string().to_string());
        });

        behavior.text_input_did_receive("hello");

        assert_eq!(*test.borrow(), "hello");

        let key = Key::new(KeyCode::Backspace, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(*test.borrow(), "hell");

        let key = Key::new(KeyCode::Z, vec![ModifierFlag::Command]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(*test.borrow(), "hello");

        let key = Key::new(KeyCode::Z, vec![ModifierFlag::Command, ModifierFlag::Shift]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(*test.borrow(), "hell");

        let key = Key::new(KeyCode::Return, vec![]);
        let press = Press::new(key);
        behavior.press_began(&press);
        behavior.press_ended(&press);

        assert_eq!(*test.borrow(), "hell\n");
    }
}

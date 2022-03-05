use crate::ui::view::WeakView;
use crate::ui::view::TextField;
use crate::ui::view::text_field::CursorMovement;
use crate::platform::history::Action;
use crate::ui::history::text_field::carat_snapshot::CaratSnapshot;

/// A reversible action that inserts text into a text field.
pub struct TextBackspace {
    view: WeakView,
    count: usize,
    cursor_movement: CursorMovement,
    cursors_before: Vec<CaratSnapshot>,
    texts_deleted: Vec<String>,
    cursors_after: Vec<CaratSnapshot>
}

impl TextBackspace {
    pub fn new(view: WeakView, count: usize, cursor_movement: CursorMovement, cursors_before: Vec<CaratSnapshot>) -> TextBackspace {
        TextBackspace {
            view,
            count,
            cursor_movement,
            cursors_before,
            texts_deleted: Vec::new(),
            cursors_after: Vec::new()
        }
    }

    fn text_field(&self) -> TextField {
        let view = self.view.upgrade().unwrap();
        TextField::from_view(view)
    }
}

impl Action for TextBackspace {
    fn name(&self) -> &str {
        "TextBackspace"
    }

    fn forward(&mut self) {
        let text_field = self.text_field();
        text_field.restore_carat_snapshots(&self.cursors_before);
        let texts_delete = text_field.backspace(self.cursor_movement.clone(), self.count);
        self.texts_deleted = texts_delete;
        self.cursors_after = text_field.carat_snapshots();
    }

    fn backward(&mut self) {
        if self.cursors_after.len() == 0 {
            return;
        }

        let text_field = self.text_field();
        let label = text_field.label();

        for (i, string) in self.texts_deleted.iter().enumerate().rev() {
            let carat_snapshot = &self.cursors_after[i];
            let index = carat_snapshot.character_index();
            label.replace_text_in_range(index..index, string);
        }
        text_field.restore_carat_snapshots(&self.cursors_before);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn merge(&self, other: &Box<dyn Action>) -> Option<Box<dyn Action>> {
        if other.name() != self.name() {
            return None
        }

        let other = other.as_any().downcast_ref::<TextBackspace>().unwrap();

        // Return early if the other action cursors don't match
        if self.cursors_after != other.cursors_before {
            return None
        }

        // Return early if the cursor movement doesn't match
        if self.cursor_movement != other.cursor_movement {
            return None
        }

        let mut combo_texts_deleted = Vec::new();

        for (index, text_deleted) in self.texts_deleted.iter().enumerate() {
            let other_text_deleted = other.texts_deleted.get(index).unwrap();
            combo_texts_deleted.push(other_text_deleted.clone() + text_deleted);
        }

        let count = self.count + other.count;
        let cursor_movement = self.cursor_movement.clone();
        let cursors_before = self.cursors_before.clone();

        let mut new = Self::new(
            self.view.clone(),
            count,
            cursor_movement,
            cursors_before
        );

        new.cursors_after = other.cursors_after.clone();
        new.texts_deleted = combo_texts_deleted;

        Some(Box::new(new))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;

    #[test]
    fn test_single_cursor_single_character() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "ab");
        assert_eq!(text_field.carat_indexes(), vec![2]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc");
        assert_eq!(text_field.carat_indexes(), vec![3]);
    }

    #[test]
    fn test_single_cursor_multiple_characters() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 2, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "a");
        assert_eq!(text_field.carat_indexes(), vec![1]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc");
        assert_eq!(text_field.carat_indexes(), vec![3]);
    }

    #[test]
    fn test_single_cursor_word() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(7, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Word, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "abc ");
        assert_eq!(text_field.carat_indexes(), vec![4]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        assert_eq!(text_field.carat_indexes(), vec![7]);
    }

    #[test]
    fn test_single_cursor_line() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(7, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Line, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "");
        assert_eq!(text_field.carat_indexes(), vec![0]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        assert_eq!(text_field.carat_indexes(), vec![7]);
    }

    #[test]
    fn test_multiple_cursors_single_character() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(1, None));
        carats.push(CaratSnapshot::new(3, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "b");
        assert_eq!(text_field.carat_indexes(), vec![0, 1]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc");
        assert_eq!(text_field.carat_indexes(), vec![1, 3]);
    }

    #[test]
    fn test_multiple_cursors_multiple_characters() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(1, None));
        carats.push(CaratSnapshot::new(3, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "b");
        assert_eq!(text_field.carat_indexes(), vec![0, 1]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc");
        assert_eq!(text_field.carat_indexes(), vec![1, 3]);
    }

    #[test]
    fn test_multiple_cursors_word() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));
        carats.push(CaratSnapshot::new(7, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Word, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), " ");
        assert_eq!(text_field.carat_indexes(), vec![0, 1]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        assert_eq!(text_field.carat_indexes(), vec![3, 7]);
    }

    #[test]
    fn test_multiple_cursors_line() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));
        carats.push(CaratSnapshot::new(7, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Line, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "");
        assert_eq!(text_field.carat_indexes(), vec![0, 0]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        assert_eq!(text_field.carat_indexes(), vec![3, 7]);
    }

    #[test]
    fn test_deletion_with_selection_single_cursor() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, Some(0..3)));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), " def");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 0);
        assert_eq!(carats[0].selection(), &None);

        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 0);
        assert_eq!(carats[0].selection(), &Some(0..3));
    }

    #[test]
    fn test_deletion_with_selection_single_cursor_when_specifying_word() {
        // In this case, we want it to ignore the word cursor movement, and
        // delete the selection only.

        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, Some(1..3)));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Word, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "a def");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 1);
        assert_eq!(carats[0].selection(), &None);

        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 3);
        assert_eq!(carats[0].selection(), &Some(1..3));
    }

    #[test]
    fn test_deletion_with_selection_single_cursor_when_specifying_line() {
        // In this case, we want it to ignore the line cursor movement, and
        // delete the selection only.

        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, Some(1..3)));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Line, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "a def");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 1);
        assert_eq!(carats[0].selection(), &None);

        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 3);
        assert_eq!(carats[0].selection(), &Some(1..3));
    }

    #[test]
    fn test_multiple_selections() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, Some(0..3)));
        carats.push(CaratSnapshot::new(3, Some(3..6)));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "f");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 2);
        assert_eq!(carats[0].character_index(), 0);
        assert_eq!(carats[0].selection(), &None);
        assert_eq!(carats[1].character_index(), 0);
        assert_eq!(carats[1].selection(), &None);

        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 2);
        assert_eq!(carats[0].character_index(), 0);
        assert_eq!(carats[0].selection(), &Some(0..3));
        assert_eq!(carats[1].character_index(), 3);
        assert_eq!(carats[1].selection(), &Some(3..6));
    }

    #[test]
    fn test_merge() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(7, None));

        let mut action1 = TextBackspace::new(
            text_field.view.downgrade(),
            4,
            CursorMovement::Character,
            carats
        );

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));

        let mut action2 = TextBackspace::new(
            text_field.view.downgrade(),
            3,
            CursorMovement::Character,
            carats
        );

        action1.forward();
        action2.forward();

        assert_eq!(text_field.label().text().string(), "");

        let boxed2: std::boxed::Box<(dyn Action + 'static)> = Box::new(action2);

        let result = action1.merge(&boxed2);
        assert!(result.is_some());

        let mut result = result.unwrap();
        result.backward();
        assert_eq!(text_field.label().text().string(), "abc def");

        result.forward();
        assert_eq!(text_field.label().text().string(), "");
    }

    #[test]
    fn test_merge_negative() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(7, None));

        let mut action1 = TextBackspace::new(
            text_field.view.downgrade(),
            4,
            CursorMovement::Character,
            carats
        );

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(2, None));

        let mut action2 = TextBackspace::new(
            text_field.view.downgrade(),
            2,
            CursorMovement::Character,
            carats
        );

        action1.forward();
        action2.forward();

        assert_eq!(text_field.label().text().string(), "c");

        let boxed2: std::boxed::Box<(dyn Action + 'static)> = Box::new(action2);

        let result = action1.merge(&boxed2);
        assert!(result.is_none());
    }
}

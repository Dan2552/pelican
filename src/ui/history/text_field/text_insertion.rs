use crate::ui::view::WeakView;
use crate::ui::view::TextField;
use crate::platform::history::Action;
use crate::ui::history::text_field::carat_snapshot::CaratSnapshot;

/// A reversible action that inserts text into a text field.
pub struct TextInsertion {
    view: WeakView,
    text: String,
    cursors_before: Vec<CaratSnapshot>,
    cursors_after: Vec<CaratSnapshot>,
    text_replaced: Vec<Option<String>>
}

impl TextInsertion {
    pub fn new(view: WeakView, text: String, cursors_before: Vec<CaratSnapshot>) -> TextInsertion {
        TextInsertion {
            view,
            text,
            cursors_before,
            cursors_after: Vec::new(),
            text_replaced: Vec::new()
        }
    }

    fn text_field(&self) -> TextField {
        let view = self.view.upgrade().unwrap();
        TextField::from_view(view)
    }
}

impl Action for TextInsertion {
    fn name(&self) -> &str {
        "TextInsertion"
    }

    fn forward(&mut self) {
        let text_field = self.text_field();
        text_field.restore_carat_snapshots(&self.cursors_before);
        self.text_replaced = text_field.insert_str(&self.text);
        self.cursors_after = text_field.carat_snapshots();
    }

    fn backward(&mut self) {
        if self.cursors_after.len() == 0 {
            return;
        }

        let text_field = self.text_field();

        for (cursor_index, text_replaced) in self.text_replaced.iter().enumerate().rev() {
            let cursor = &self.cursors_after[cursor_index];
            let start = cursor.character_index() - self.text.len();
            let end = cursor.character_index();

            if let Some(text_replaced) = text_replaced {
                text_field.label().replace_text_in_range(start..end, text_replaced);
            } else {
                text_field.label().replace_text_in_range(start..end, "");
            }
        }

        text_field.restore_carat_snapshots(&self.cursors_before);
    }

    fn merge(&self, other: &Box<dyn Action>) -> Option<Box<dyn Action>> {
        if other.name() != self.name() {
            return None
        }

        let other = other.as_any().downcast_ref::<TextInsertion>().unwrap();

        // Return early if the other action cursors don't match
        if self.cursors_after != other.cursors_before {
            return None
        }

        let text = self.text.clone() + &other.text;

        let mut combo_text_replaced: Vec<Option<String>> = Vec::new();

        for (index, text_replaced) in self.text_replaced.iter().enumerate() {
            if let Some(text_replaced) = text_replaced {
                if let Some(other_text_replaced) = other.text_replaced.get(index) {
                    if let Some(other_text_replaced) = other_text_replaced {
                        combo_text_replaced.push(Some(text_replaced.clone() + other_text_replaced));
                    } else {
                        combo_text_replaced.push(Some(text_replaced.clone()));
                    }
                    combo_text_replaced.push(Some(text_replaced.clone()));
                } else {
                    combo_text_replaced.push(Some(text_replaced.clone()));
                }
            } else {
                combo_text_replaced.push(None);
            }
        }

        let mut new = Self::new(
            self.view.clone(),
            text,
            self.cursors_before.clone(),
        );

        new.cursors_after = other.cursors_after.clone();
        new.text_replaced = combo_text_replaced;
        Some(Box::new(new))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;

    #[test]
    fn test_forward() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello");
        assert_eq!(text_field.carat_indexes(), vec![5]);
    }

    #[test]
    fn test_forward_multi_cursor() {
        let frame = Rectangle::new(0, 0, 100, 100);

        let text_field = TextField::new(frame, "|".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));
        carats.push(CaratSnapshot::new(1, None));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );


        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello|Hello");
        assert_eq!(text_field.carat_indexes(), vec![5, 11]);
    }

    #[test]
    fn test_backward() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion.forward();
        text_insertion.backward();

        assert_eq!(text_field.label().text().string(), "");
        assert_eq!(text_field.carat_indexes(), vec![0]);
    }

    #[test]
    fn test_backward_multi_cursor() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "|".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));
        carats.push(CaratSnapshot::new(1, None));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello|Hello");

        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 2);
        assert_eq!(carats[0].character_index(), 5);
        assert_eq!(carats[0].selection(), &None);
        assert_eq!(carats[1].character_index(), 11);
        assert_eq!(carats[1].selection(), &None);

        text_insertion.backward();

        assert_eq!(text_field.label().text().string(), "|");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 2);
        assert_eq!(carats[0].character_index(), 0);
        assert_eq!(carats[0].selection(), &None);
        assert_eq!(carats[1].character_index(), 1);
        assert_eq!(carats[1].selection(), &None);
    }

    #[test]
    fn test_insertion_with_selection() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "Hi world".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, Some(0..2)));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello world");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 5);
        assert_eq!(carats[0].selection(), &None);

        text_insertion.backward();

        assert_eq!(text_field.label().text().string(), "Hi world");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 0);
        assert_eq!(carats[0].selection(), &Some(0..2));
    }

    #[test]
    fn test_merge() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut text_insertion1 = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(5, None));

        let mut text_insertion2 = TextInsertion::new(
            text_field.view.downgrade(),
            " world".to_string(),
            carats
        );

        text_insertion1.forward();
        text_insertion2.forward();

        let boxed2: std::boxed::Box<(dyn Action + 'static)> = Box::new(text_insertion2);

        let result = text_insertion1.merge(&boxed2);
        assert!(result.is_some());
        let mut result = result.unwrap();
        result.backward();
        assert_eq!(text_field.label().text().string(), "");

        result.forward();
        assert_eq!(text_field.label().text().string(), "Hello world");
    }

    #[test]
    fn test_merge_negative() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut text_insertion1 = TextInsertion::new(
            text_field.view.downgrade(),
            " world".to_string(),
            carats
        );

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut text_insertion2 = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion1.forward();
        text_insertion2.forward();

        assert_eq!(text_field.label().text().string(), "Hello world");

        let boxed2: std::boxed::Box<(dyn Action + 'static)> = Box::new(text_insertion2);
        let result = text_insertion1.merge(&boxed2);

        assert!(result.is_none());
    }
}

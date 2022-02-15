use crate::ui::view::WeakView;
use crate::ui::view::TextField;
use crate::platform::history::Action;
use crate::ui::history::text_field::CaratSnapshot;

struct TextInsertion {
    view: WeakView,
    text: String,
    cursors_before: Vec<CaratSnapshot>,
    cursors_after: Vec<CaratSnapshot>

    // TODO: what if a cursor had a Selection before the insertion?
}

impl TextInsertion {
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
        text_field.insert_str(&self.text);
        self.cursors_after = text_field.carat_snapshots();
    }

    fn backward(&mut self) {
        if self.cursors_after.len() == 0 {
            return;
        }

        let text_field = self.text_field();
        text_field.restore_carat_snapshots(&self.cursors_after);
        text_field.delete_characters(self.text.len());
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

        let mut text_insertion = TextInsertion {
            view: text_field.view.downgrade(),
            text: "Hello".to_string(),
            cursors_before: vec![CaratSnapshot::new(0, None)],
            cursors_after: vec![]
        };

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello");
        assert_eq!(text_field.carat_indexes(), vec![5]);
    }

    #[test]
    fn test_forward_multi_cursor() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "|".to_string());

        let mut text_insertion = TextInsertion {
            view: text_field.view.downgrade(),
            text: "Hello".to_string(),
            cursors_before: vec![CaratSnapshot::new(0, None), CaratSnapshot::new(1, None)],
            cursors_after: vec![]
        };

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello|Hello");
        assert_eq!(text_field.carat_indexes(), vec![5, 11]);
    }

    #[test]
    fn test_backward() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());

        let mut text_insertion = TextInsertion {
            view: text_field.view.downgrade(),
            text: "Hello".to_string(),
            cursors_before: vec![CaratSnapshot::new(0, None)],
            cursors_after: vec![]
        };

        text_insertion.forward();
        text_insertion.backward();

        assert_eq!(text_field.label().text().string(), "");
        assert_eq!(text_field.carat_indexes(), vec![0]);
    }

    #[test]
    fn test_backward_multi_cursor() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "|".to_string());

        let mut text_insertion = TextInsertion {
            view: text_field.view.downgrade(),
            text: "Hello".to_string(),
            cursors_before: vec![CaratSnapshot::new(0, None), CaratSnapshot::new(1, None)],
            cursors_after: vec![]
        };

        text_insertion.forward();
        text_insertion.backward();

        assert_eq!(text_field.label().text().string(), "|");
        assert_eq!(text_field.carat_indexes(), vec![0, 1]);
    }

}

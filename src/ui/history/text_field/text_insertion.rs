use crate::ui::view::WeakView;
use crate::ui::view::TextField;
use crate::platform::history::Action;

struct TextInsertion {
    view: WeakView,
    text: String,
    cursors_before: Vec<usize>
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
        text_field.set_carat_indexes(&self.cursors_before);
        text_field.insert_str(&self.text);
    }

    fn backward(&mut self) {
        let text_field = self.text_field();
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
            cursors_before: vec![0]
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
            cursors_before: vec![0, 1]
        };

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello|Hello");
        assert_eq!(text_field.carat_indexes(), vec![5, 11]);
    }

    #[test]
    fn test_backward() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "Hello".to_string());

        let mut text_insertion = TextInsertion {
            view: text_field.view.downgrade(),
            text: "Hello".to_string(),
            cursors_before: vec![0]
        };

        text_insertion.backward();

        assert_eq!(text_field.label().text().string(), "");
        assert_eq!(text_field.carat_indexes(), vec![0]);
    }
}

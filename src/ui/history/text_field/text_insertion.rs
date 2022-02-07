use crate::ui::view::WeakView;
use crate::ui::view::TextField;
use crate::platform::history::Action;

struct TextInsertion {
    view: WeakView,
    text: String,
    index: usize
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
        text_field.insert_str(self.index, &self.text);
    }

    fn backward(&mut self) {
        let text_field = self.text_field();
        let start = self.index;
        let end = start + self.text.len();
        text_field.replace_range(start..end, "");
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
            index: 0
        };

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello");
    }
}

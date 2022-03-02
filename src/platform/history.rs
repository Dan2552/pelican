/// A trait which represents a single action in a history.
///
/// For example, a `TextFieldEditAction` would repesent a user interaction that
/// can be undone/redone.
pub trait Action {
    fn name(&self) -> &str;
    fn forward(&mut self);
    fn backward(&mut self);
    fn merge(&self, _other: Box<dyn Action>) -> Option<Box<dyn Action>> {
        None
    }
}

/// A history of actions which can be undone/redone.
///
/// E.g. a `pelican::ui::TextField` would create a `History` for itself.
pub struct History {
    actions: Vec<Box<dyn Action>>,
    current: usize,
}

impl History {
    /// Create a new history.
    pub fn new() -> History {
        History {
            actions: Vec::new(),
            current: 0,
        }
    }

    /// Add an action to the history.
    pub fn add(&mut self, action: Box<dyn Action>) {
        if self.current < self.actions.len() {
            self.actions.truncate(self.current);
        }
        self.actions.push(action);
        self.current = self.actions.len();
    }

    /// Undo the last action.
    pub fn undo(&mut self) {
        if self.current > 0 {
            self.actions[self.current - 1].backward();
            self.current -= 1;
        }
    }

    /// Redo the last undone action.
    pub fn redo(&mut self) {
        if self.current < self.actions.len() {
            self.actions[self.current].forward();
            self.current += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::Text;
    use std::rc::Rc;
    use std::cell::RefCell;

    struct TestExampleRef {
        test_example: Rc<RefCell<TestExample>>
    }

    impl Clone for TestExampleRef {
        fn clone(&self) -> Self {
            TestExampleRef {
                test_example: self.test_example.clone()
            }
        }
    }

    struct TestExample {
        text: Text
    }

    struct TestAction {
        example: TestExampleRef,
        addition: String,
        index: usize,
    }

    impl Action for TestAction {
        fn name(&self) -> &str {
            "TestAction"
        }

        fn forward(&mut self) {
            let mut example = self.example.test_example.borrow_mut();
            example.text.insert_str(self.index, &self.addition);
        }

        fn backward(&mut self) {
            let mut example = self.example.test_example.borrow_mut();
            let start = self.index;
            let end = start + self.addition.len();
            example.text.replace_range(start..end, "");
        }
    }

    #[test]
    fn test_history() {
        let example = TestExampleRef {
            test_example: Rc::new(RefCell::new(TestExample {
                text: Text::from("")
            }))
        };

        let mut action1 = TestAction {
            example: example.clone(),
            addition: "Hello".to_string(),
            index: 0
        };

        let mut action2 = TestAction {
            example: example.clone(),
            addition: "World".to_string(),
            index: 5
        };

        action1.forward();
        action2.forward();

        let mut history = History::new();
        history.add(Box::new(action1));
        history.add(Box::new(action2));

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "HelloWorld");
        }

        history.undo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "Hello");
        }

        history.redo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "HelloWorld");
        }

        history.undo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "Hello");
        }

        history.undo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "");
        }

        history.undo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "");
        }

        history.redo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "Hello");
        }

        history.redo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "HelloWorld");
        }

        history.redo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "HelloWorld");
        }
    }

    #[test]
    fn test_change_of_history() {
        let example = TestExampleRef {
            test_example: Rc::new(RefCell::new(TestExample {
                text: Text::from("")
            }))
        };

        let mut action1 = TestAction {
            example: example.clone(),
            addition: "Hello".to_string(),
            index: 0
        };

        let mut action2 = TestAction {
            example: example.clone(),
            addition: "World".to_string(),
            index: 5
        };

        let mut action3 = TestAction {
            example: example.clone(),
            addition: "Universe".to_string(),
            index: 5
        };

        action1.forward();
        action2.forward();

        let mut history = History::new();
        history.add(Box::new(action1));
        history.add(Box::new(action2));

        assert_eq!(history.actions.len(), 2);

        history.undo();

        assert_eq!(history.actions.len(), 2);

        action3.forward();

        history.add(Box::new(action3));

        assert_eq!(history.actions.len(), 2);

        history.undo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "Hello");
        }

        history.redo();

        {
            let example = example.test_example.borrow();
            assert_eq!(example.text.to_string(), "HelloUniverse");
        }
    }
}

use crate::ui::view::{View, Behavior, ViewInner};
use std::rc::Weak;
use std::cell::RefCell;

pub struct WeakView {
    pub(crate) inner_self: Weak<RefCell<ViewInner>>,
    pub(crate) behavior: Weak<RefCell<Box<dyn Behavior>>>,
    pub debug_name: String
}

impl WeakView {
    pub fn upgrade(&self) -> Option<View> {
        if let Some(inner_self) = self.inner_self.upgrade() {
            if let Some(behavior) = self.behavior.upgrade() {
                Some(View {
                    inner_self: inner_self,
                    behavior: behavior,
                    debug_name: self.debug_name.clone()
                })
            } else {
                panic!("Inner self present but behavior is missing");
            }
        } else {
            None
        }
    }

    /// An empty WeakView. When trying to `upgrade()`, the `Option` result will
    /// be `None`.
    pub fn none() -> WeakView {
        WeakView {
            inner_self: Weak::new(),
            behavior: Weak::new(),
            debug_name: String::from("none")
        }
    }

    pub fn is_none(&self) -> bool {
        if let Some(_) = self.upgrade() {
            false
        } else {
            true
        }
    }

    pub fn id(&self) -> Option<usize> {
        if let Some(inner_self) = self.inner_self.upgrade() {
            Some(inner_self.borrow().id)
        } else {
            None
        }
    }
}

impl Clone for WeakView {
    fn clone(&self) -> Self {
        WeakView {
            inner_self: self.inner_self.clone(),
            behavior: self.behavior.clone(),
            debug_name: self.debug_name.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;

    #[test]
    fn test_weak_view_upgrade() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let view = View::new(frame);
        let weak_view = view.downgrade();
        assert!(weak_view.upgrade().is_some());
        assert_eq!(weak_view.upgrade().unwrap(), view);
    }

    #[test]
    fn test_weak_view_is_none() {
        let weak_view = WeakView::none();
        assert!(weak_view.is_none());
    }

    #[test]
    fn test_clone() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let view = View::new(frame);
        let weak_view = view.downgrade();
        let weak_view_clone = weak_view.clone();
        assert!(weak_view_clone.upgrade().is_some());
        assert_eq!(weak_view_clone.upgrade().unwrap(), view);
    }
}

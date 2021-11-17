use crate::ui::view::{View, Behavior, ViewInner};
use std::rc::Weak;
use std::cell::RefCell;

pub struct WeakView {
    pub id: uuid::Uuid,
    pub(crate) inner_self: Weak<RefCell<ViewInner>>,
    pub(crate) behavior: Weak<RefCell<Box<dyn Behavior>>>
}

impl WeakView {
    pub fn upgrade(&self) -> Option<View> {
        if let Some(inner_self) = self.inner_self.upgrade() {
            if let Some(behavior) = self.behavior.upgrade() {
                Some(View {
                    id: self.id,
                    inner_self: inner_self,
                    behavior: behavior
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
            id: uuid::Uuid::new_v4(),
            inner_self: Weak::new(),
            behavior: Weak::new()
        }
    }

    pub fn is_none(&self) -> bool {
        if let Some(_) = self.upgrade() {
            true
        } else {
            false
        }
    }
}

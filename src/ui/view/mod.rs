mod behavior;
mod default_behavior;
mod view_inner;
mod view;
mod weak_view;
mod delegate;
mod image_view;
mod label;
mod scroll_view;

pub use view::View;
pub use weak_view::WeakView;
pub use behavior::Behavior;
pub use default_behavior::DefaultBehavior;
pub(crate) use view_inner::ViewInner;
pub use image_view::ImageView;
pub use label::Label;
pub use scroll_view::ScrollView;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;
    use crate::graphics::Point;
    use crate::graphics::Size;

    #[test]
    /// When there are no more strong references, the weak view can no longer be
    /// upgraded (it becomes a `None` on `upgrade()`).
    fn weak_dying() {
        let mut weak = WeakView::none();
        assert_eq!(weak.upgrade().is_some(), false);

        {
            let frame = Rectangle {
                origin: Point { x: 10, y: 10 },
                size: Size { width: 50, height: 50 }
            };
            let strong = View::new(frame.clone());
            weak = strong.downgrade();

            assert_eq!(weak.upgrade().is_some(), true)
        }

        assert_eq!(weak.upgrade().is_some(), false)
    }

    #[test]
    /// A WeakView instantiated with `none()` is a `None` on `upgrade()`.
    fn weak_view_upgrade() {
        let weak_view = WeakView::none();

        assert!(weak_view.upgrade().is_none());
    }

    #[test]
    /// Checks that `id` is consistent between `View`, `WeakView` and clones,
    /// but not other instances.
    fn strong_vs_weak_ids() {
        let frame = Rectangle {
            origin: Point { x: 10, y: 10 },
            size: Size { width: 50, height: 50 }
        };

        let strong = View::new(frame.clone());
        let weak = strong.downgrade();
        let strong_again = weak.upgrade().unwrap();
        let strong_clone = strong.clone();

        assert_eq!(strong.id, weak.id);
        assert_eq!(weak.id, strong_again.id);
        assert_eq!(strong.id, strong_clone.id);

        let frame = Rectangle {
            origin: Point { x: 10, y: 10 },
            size: Size { width: 50, height: 50 }
        };

        let different = View::new(frame.clone());

        assert_ne!(strong.id, different.id);
    }

    #[test]
    /// Tests add_subview and superview
    fn parent_child_relationship() {
        let frame = Rectangle {
            origin: Point { x: 10, y: 10 },
            size: Size { width: 50, height: 50 }
        };

        let view_parent = View::new(frame.clone());
        let view_child = View::new(frame.clone());

        view_parent.add_subview(view_child.clone());

        let view_child1 = view_child.clone();
        let child_inner_self = &view_child1.inner_self.borrow();
        let childs_parent = &child_inner_self.superview;

        assert_eq!(view_parent, childs_parent.upgrade().unwrap());

        let inner_self = view_parent.inner_self.borrow();
        let contains_child = inner_self.subviews.contains(&view_child);
        assert_eq!(contains_child, true);
    }
}

use crate::graphics::Rectangle;
use crate::graphics::Point;
use crate::graphics::Layer;
use crate::ui::Color;
use crate::graphics::LayerDelegate;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub struct View {
    /// Some way to compare `View`s (`==`) and `WeakView`s
    id: uuid::Uuid,

    /// The actual view, wrapped in a reference count, so that this `View`
    /// object can easily be copied around (`clone()`).
    inner_self: Rc<RefCell<ViewInner>>
}

struct WeakView {
    id: uuid::Uuid,
    inner_self: Weak<RefCell<ViewInner>>
}

impl Clone for View {
    fn clone(&self) -> Self {
      View {
          id: self.id.clone(),
          inner_self: self.inner_self.clone()
      }
    }
}

impl PartialEq for View {
    fn eq(&self, rhs: &View) -> bool {
        self.id == rhs.id
    }
}

impl std::fmt::Debug for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.id.to_string();

        f.debug_tuple("")
         .field(&id)
         .finish()
    }
}

impl View {
    pub fn new(frame: Rectangle) -> View {
        View {
            id: uuid::Uuid::new_v4(),
            inner_self: Rc::new(RefCell::new(ViewInner::new(frame)))
        }
    }

    /// Adds a child `View` to this `View`.
    ///
    /// Also sets the parent (`superview`) of the child view to this `View`.
    pub fn add_subview(&mut self, child: View) {
        let weak_self = self.downgrade();
        let mut inner_self = self.inner_self.borrow_mut();

        inner_self.add_subview(weak_self, child);
    }

    /// Get a weak reference (`WeakView`) for this `View`
    ///
    /// E.g. used to refer to a superview to not cause a cyclic reference.
    fn downgrade(&self) -> WeakView {
        let weak_inner = Rc::downgrade(&self.inner_self);

        WeakView {
            id: self.id,
            inner_self: weak_inner
        }
    }

    /// The parent view; the view that contains (and owns) this one.
    fn superview(&self) -> Option<View> {
        let inner_self = self.inner_self.borrow();
        inner_self.superview.upgrade()
    }

    /// Children views; views that are contained (and owned) within this view.
    fn subviews(&self) -> Vec<View> {
        let inner_self = self.inner_self.borrow();
        inner_self.subviews.clone()
    }

    /// Request for this view to be redrawn soon.
    ///
    /// See `#draw`, which includes the instructions on what would actually be
    /// drawn to screen.
    fn set_needs_display(&self) {
        let mut inner_self = self.inner_self.borrow_mut();
        inner_self.set_needs_display();
    }

    /// Defines what actually gets drawn to screen to represent this view.
    ///
    /// For example, the default `View` implementation simply draws the
    /// background color as a box of the size of the frame.
    fn draw(&self) {
        let mut inner_self = self.inner_self.borrow_mut();
        inner_self.draw();
    }

    /// Change the background color for this view.
    pub fn set_background_color(&self, color: Color) {
        let mut inner_self = self.inner_self.borrow_mut();
        inner_self.set_background_color(color);
    }
}

impl WeakView {
    /// An empty WeakView. When trying to `upgrade()`, the `Option` result will
    /// be `None`.
    fn none() -> WeakView {
        WeakView {
            id: uuid::Uuid::new_v4(),
            inner_self: Weak::new()
        }
    }

    fn upgrade(&self) -> Option<View> {
        if let Some(inner_self) = self.inner_self.upgrade() {
            Some(View {
                id: self.id,
                inner_self: inner_self
            })
        } else {
            None
        }
    }
}

struct ViewInner {
    /// The size and position (within its superview) of this View.
    ///
    /// Used for placing the view in the parent.
    frame: Rectangle,

    /// The size and position of the View from the view's own coordinate
    /// perspective.
    ///
    /// Will commonly have the same size as `frame`, but in most circumstances
    /// the position will be `0,0`.
    ///
    /// When the position is changed, the internal contents will move rather
    /// than the View itself. For example, this could be used to create behavior
    /// like a scroll view. E.g. if an image were inside this view, it could be
    /// used to pan the image.
    ///
    /// If you still don't get it, see:
    /// https://stackoverflow.com/a/28917673/869367
    bounds: Rectangle,

    /// The background color of the view. In its simplest form, a View is just a
    /// rectangle with a single color - this is that color.
    background_color: Color,

    /// The z position. A view with a higher number compared to its sibling
    /// views (the same superview) will result in the view being drawn infront
    /// of the others.
    z_index: u32,

    /// The actual drawable canvas from the `graphics` library.
    ///
    /// Think of the View as instructions or a template for a picture (this
    /// behavior itself defined in `#draw`), and then the `layer` is the canvas
    /// that picture is drawn onto.
    ///
    /// The layer will also handle lifecycle of when the view is to be drawn.
    /// That is to say, the layer will call this view (it's `delegate`) to draw
    /// when the platform calls for it to be drawn (the `layer` itself will be
    /// the thing calling `#draw` for this view).
    layer: Option<Layer>,

    /// The parent view; the view that contains (and owns) this one.
    superview: WeakView,

    /// Children views; views that are contained (and owned) within this view.
    subviews: Vec<View>,

    /// Whether this view is visible or not. When hidden at the next render to
    /// screen, it'll behave the same as if it were not in the view hierarchy at
    /// all.
    hidden: bool
}

impl ViewInner {
    fn new(frame: Rectangle) -> ViewInner {
        let white = Color::white();

        let bounds = Rectangle {
            position: Point { x: 0, y: 0 },
            size: frame.size.clone()
        };

        ViewInner {
            frame: frame,
            bounds: bounds,
            background_color: white,
            z_index: 0,
            layer: None,
            superview: WeakView::none(),
            subviews: Vec::new(),
            hidden: false,
            // _view_delegate: None
        }
    }

    fn set_needs_display(&mut self) {
        if let Some(layer) = &mut self.layer {
            if layer.get_needs_display() {
                return;
            }

            layer.set_needs_display();

            if let Some(superview) = self.superview.upgrade() {
                superview.set_needs_display();
            }
        }
    }

    pub fn draw(&mut self) {
        if let Some(layer) = &mut self.layer {
            layer.clear_with_color(self.background_color.to_graphics_color());
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.set_needs_display();
    }

    /// * Gives ownership to child to self
    /// * Makes the weak_self the parent of the child
    fn add_subview(&mut self, weak_self: WeakView, child: View) {
        {
            let mut child_inner = child.inner_self.borrow_mut();
            child_inner.superview = weak_self;
        }

        self.subviews.push(child);

    }
}

impl LayerDelegate for View {
    fn layer_will_draw(&mut self, layer: &Layer) {

    }

    fn draw_layer(&mut self, layer: &Layer) {
        // if let Some(self_layer) = &self.layer {
        //     if layer != self_layer {
        //         // TODO: change this to return once known `layer != self_layer` works as expected
        //         panic!("layer mismatch");
        //         return;
        //     }
        //     self.draw();
        // }
    }
}

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

        {
            let frame = Rectangle {
                position: Point { x: 10, y: 10 },
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

        assert_eq!(weak_view.upgrade(), None);
    }

    #[test]
    /// Checks that `id` is consistent between `View`, `WeakView` and clones,
    /// but not other instances.
    fn strong_vs_weak_ids() {
        let frame = Rectangle {
            position: Point { x: 10, y: 10 },
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
            position: Point { x: 10, y: 10 },
            size: Size { width: 50, height: 50 }
        };

        let different = View::new(frame.clone());

        assert_ne!(strong.id, different.id);
    }

    #[test]
    /// Tests add_subview and superview
    fn parent_child_relationship() {
        let frame = Rectangle {
            position: Point { x: 10, y: 10 },
            size: Size { width: 50, height: 50 }
        };

        let mut view_parent = View::new(frame.clone());
        let view_child = View::new(frame.clone());

        view_parent.add_subview(view_child.clone());

        assert_eq!(view_parent, view_child.superview().unwrap());

        let contains_child = view_parent.subviews().contains(&view_child);

        assert_eq!(contains_child, true);
    }
}

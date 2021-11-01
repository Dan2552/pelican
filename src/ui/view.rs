use crate::graphics::Rectangle;
use crate::graphics::Point;
use crate::graphics::Layer;
use crate::ui::Color;
use crate::graphics::LayerDelegate;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub trait WeakViewBehavior {
    fn upgrade(&self) -> Option<Box<dyn ViewBehavior>>;
}

pub trait ViewBehavior {
    fn data(&self) -> View;
    fn add_subview(&mut self, child: Child) {
        self.data().add_subview(child);
    }

    fn set_needs_display(&self) {
        self.data().set_needs_display();
    }

    fn draw(&self) {
        self.data().draw();
    }

    fn set_background_color(&self, color: Color) {
        self.data().set_background_color(color);
    }

    fn set_hidden(&self, value: bool) {
        self.data().set_hidden(value);
    }
}

pub struct ViewParent {
    pub(crate) view: Box<dyn WeakViewBehavior>
}

pub struct Child {
    pub(crate) view: Box<dyn ViewBehavior>
}

impl PartialEq for Child {
    fn eq(&self, rhs: &Child) -> bool {
        let lhs_view = self.view.data();
        let rhs_view = rhs.view.data();

        lhs_view.id == rhs_view.id
    }
}

pub struct View {
    /// Some way to compare `View`s (`==`) and `WeakView`s
    pub id: uuid::Uuid,

    /// The actual view, wrapped in a reference count, so that this `View`
    /// object can easily be copied around (`clone()`).
    pub(crate) inner_self: Rc<RefCell<ViewInner>>
}

pub(crate) struct ViewInner {
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
    pub(crate) superview: Option<ViewParent>,

    /// Children views; views that are contained (and owned) within this view.
    pub(crate) subviews: Vec<Child>,

    /// Whether this view is visible or not. When hidden at the next render to
    /// screen, it'll behave the same as if it were not in the view hierarchy at
    /// all.
    hidden: bool
}

trait ViewDelegate {

}

impl View {
    pub fn new(frame: Rectangle) -> Self {
        let white = Color::white();

        let bounds = Rectangle {
            position: Point { x: 0, y: 0 },
            size: frame.size.clone()
        };

        let inner_self = ViewInner {
            frame: frame,
            bounds: bounds,
            background_color: white,
            z_index: 0,
            layer: None,
            superview: None,
            subviews: Vec::new(),
            hidden: false
        };

        View {
            id: uuid::Uuid::new_v4(),
            inner_self: Rc::new(RefCell::new(inner_self))
        }
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
}

impl ViewBehavior for View {
    fn data(&self) -> View {
        self.clone()
    }

    /// Adds a child `View` to this `View`.
    ///
    /// Also sets the parent (`superview`) of the child view to this `View`.
    fn add_subview(&mut self, child: Child) {
        let weak_self = self.downgrade();
        let mut inner_self = self.inner_self.borrow_mut();

        // inner_self.add_subview(weak_self, child);

        {
            let view = child.view.data();
            let mut child_inner = view.inner_self.borrow_mut();
            child_inner.superview = Some(ViewParent { view: Box::new(weak_self) });
        }

        inner_self.subviews.push(child);
    }

    /// Request for this view to be redrawn soon.
    ///
    /// See `#draw`, which includes the instructions on what would actually be
    /// drawn to screen.
    fn set_needs_display(&self) {
        let mut inner_self = self.inner_self.borrow_mut();

        if let Some(layer) = &mut inner_self.layer {
            if layer.get_needs_display() {
                return;
            }

            layer.set_needs_display();

            if let Some(parent) = &inner_self.superview {


                if let Some(superview) = parent.view.upgrade() {
                    superview.set_needs_display();
                }
            }
        }
    }

    /// Defines what actually gets drawn to screen to represent this view.
    ///
    /// For example, the default `View` implementation simply draws the
    /// background color as a box of the size of the frame.
    fn draw(&self) {
        let mut inner_self = self.inner_self.borrow_mut();
        // inner_self.draw();

        let color = inner_self.background_color.to_graphics_color();

        if let Some(layer) = &mut inner_self.layer {
            layer.clear_with_color(color);
        }
    }

    /// Change the background color for this view.
    fn set_background_color(&self, color: Color) {
        let mut inner_self = self.inner_self.borrow_mut();

        inner_self.background_color = color;
        self.set_needs_display();
    }

    fn set_hidden(&self, value: bool) {
        let mut inner_self = self.inner_self.borrow_mut();
        inner_self.hidden = value;
    }
}

struct WeakView {
    id: uuid::Uuid,
    inner_self: Weak<RefCell<ViewInner>>
}

impl WeakViewBehavior for WeakView {
    fn upgrade(&self) -> Option<Box<dyn ViewBehavior>> {
        if let Some(inner_self) = self.inner_self.upgrade() {
            Some(Box::new(View {
                id: self.id,
                inner_self: inner_self
            }))
        } else {
            None
        }
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

        assert!(weak_view.upgrade().is_none());
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
        assert_eq!(weak.id, strong_again.data().id);
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

        view_parent.add_subview(Child { view: Box::new(view_child.clone()) });

        let view_child1 = view_child.clone();
        let child_inner_self = &view_child1.inner_self.borrow();
        let childs_parent = child_inner_self.superview.as_ref();
        let childs_parent = &childs_parent.unwrap().view;

        assert_eq!(view_parent, childs_parent.upgrade().unwrap().data());

        let inner_self = view_parent.inner_self.borrow();
        let contains_child = inner_self.subviews.contains(&Child { view: Box::new(view_child) });
        assert_eq!(contains_child, true);
    }
}

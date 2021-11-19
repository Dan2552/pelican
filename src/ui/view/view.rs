use crate::ui::Color;
use crate::ui::view::{WeakView, Behavior, DefaultBehavior, ViewInner};
use crate::graphics::{Layer, Rectangle, Point, LayerDelegate};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct View {
    /// Some way to compare `View`s (`==`) and `WeakView`s
    pub id: uuid::Uuid,

    /// The actual view, wrapped in a reference count, so that this `View`
    /// object can easily be copied around (`clone()`).
    pub(crate) inner_self: Rc<RefCell<ViewInner>>,

    /// The behavior for this view. This is essentially used in order to allow
    /// inheritance-alike functionality while being able to refer to differently
    /// implemented objects all as `View`.
    ///
    /// The default constructor for `View` uses the `DefaultBehavior` struct.
    pub(crate) behavior: Rc<RefCell<Box<dyn Behavior>>>
}

impl View {
    pub fn new(frame: Rectangle) -> Self {
        let behavior = DefaultBehavior {
            view: WeakView::none()
        };

        let view = View::new_with_behavior(Box::new(behavior), frame);

        view
    }

    pub fn new_with_behavior(behavior: Box<dyn Behavior>, frame: Rectangle) -> Self {
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
            superview: WeakView::none(),
            subviews: Vec::new(),
            hidden: false
        };

        let view = View {
            id: uuid::Uuid::new_v4(),
            inner_self: Rc::new(RefCell::new(inner_self)),
            behavior: Rc::new(RefCell::new(behavior))
        };

        {
            let view = view.clone();
            let mut behavior = view.behavior.borrow_mut();
            behavior.set_view(view.downgrade());
            behavior.set_super_behavior_view(view.clone());
        }

        view
    }

    pub fn add_subview(&self, child: View) {
        let behavior = self.behavior.borrow();
        behavior.add_subview(child);
    }

    pub(crate) fn set_needs_display(&self) {
        let behavior = self.behavior.borrow();
        behavior.set_needs_display();
    }

    fn draw(&self) {
        let behavior = self.behavior.borrow();
        behavior.draw();
    }

    pub fn set_background_color(&self, color: Color) {
        let behavior = self.behavior.borrow();
        behavior.set_background_color(color);
    }

    pub fn set_hidden(&self, value: bool) {
        let behavior = self.behavior.borrow();
        behavior.set_hidden(value);
    }

    pub fn is_window(&self) -> bool {
        let behavior = self.behavior.borrow();
        behavior.is_window()
    }

    pub fn get_frame(&self) -> Rectangle {
        let inner_self = self.inner_self.borrow();
        inner_self.frame.clone()
    }

    /// Get a weak reference (`WeakView`) for this `View`
    ///
    /// E.g. used to refer to a superview to not cause a cyclic reference.
    pub fn downgrade(&self) -> WeakView {
        let weak_inner = Rc::downgrade(&self.inner_self);
        let weak_behavior = Rc::downgrade(&self.behavior);

        WeakView {
            id: self.id,
            inner_self: weak_inner,
            behavior: weak_behavior
        }
    }

    pub fn superview(&self) -> WeakView {
        let inner_self = self.inner_self.borrow();

        if let Some(superview) = inner_self.superview.upgrade() {
            superview.downgrade()
        } else {
            WeakView::none()
        }
    }

    pub fn subviews(&self) -> Vec<View> {
        let inner_self = self.inner_self.borrow();
        inner_self.subviews.clone()
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
          inner_self: self.inner_self.clone(),
          behavior: self.behavior.clone()
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

use crate::ui::Color;
use crate::ui::Touch;
use crate::ui::view::{WeakView, Behavior, DefaultBehavior, ViewInner};
use crate::graphics::{Layer, Rectangle, Point, LayerDelegate};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::cell::Ref;
use crate::ui::gesture::recognizer::Recognizer;
use crate::ui::event::TouchEvent;

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
    pub(crate) behavior: Rc<RefCell<Box<dyn Behavior>>>,

    pub debug_name: String
}

impl View {
    pub fn new(frame: Rectangle<i32, u32>) -> Self {
        let behavior = DefaultBehavior {
            view: WeakView::none()
        };

        let view = View::new_with_behavior(Box::new(behavior), frame, "plainview");

        view
    }

    pub fn new_with_behavior(behavior: Box<dyn Behavior>, frame: Rectangle<i32, u32>, debug_name: &str) -> Self {
        let white = Color::white();

        let bounds = Rectangle {
            origin: Point { x: 0, y: 0 },
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
            gesture_recognizers: Vec::new(),
            hidden: false,
            user_interaction_enabled: true
        };

        let view = View {
            id: uuid::Uuid::new_v4(),
            inner_self: Rc::new(RefCell::new(inner_self)),
            behavior: Rc::new(RefCell::new(behavior)),
            debug_name: String::from(debug_name)
        };

        {
            let view = view.clone();
            let mut behavior = view.behavior.borrow_mut();
            behavior.set_view(view.downgrade());
            behavior.set_super_behavior_view(view.clone());
        }

        view
    }

    /// Adds a child `View` to this `View`.
    ///
    /// Also sets the parent (`superview`) of the child view to this `View`.
    pub fn add_subview(&self, child: View) {
        let weak_self = self.downgrade();
        let mut inner_self = self.inner_self.borrow_mut();

        {
            let mut child_inner = child.inner_self.borrow_mut();

            // Set the child superview
            child_inner.superview = weak_self;
        }

        inner_self.subviews.push(child.clone());

        child.set_needs_display();
    }

    /// Remove the view from its superview.
    pub fn remove_from_superview(&self) {
        let inner_self = self.inner_self.borrow();

        if let Some(superview) = inner_self.superview.upgrade() {
            {
                let mut superview_inner = superview.inner_self.borrow_mut();
                superview_inner.subviews.retain(|view| view.id != self.id);
            }
            superview.set_needs_display();
        }
    }

    /// Add a gesture recognizer to the view.
    /// TODO: Ref here means that inner_self is still borrowed, which means
    /// other things can't borrow mut it.
    pub fn add_gesture_recognizer(&self, gesture_recognizer: Box<dyn Recognizer>) {
        let mut inner_self = self.inner_self.borrow_mut();
        gesture_recognizer.set_view(self.downgrade());
        inner_self.gesture_recognizers.push(Rc::new(gesture_recognizer));
    }

    pub fn gesture_recognizers(&self) -> Vec<Weak<Box<dyn Recognizer>>> {
        let inner_self = self.inner_self.borrow();
        inner_self.gesture_recognizers
            .iter()
            .map(|recognizer| Rc::downgrade(recognizer)).collect()
    }

    fn draw(&self) {
        let behavior = self.behavior.borrow();
        behavior.draw();
    }

    /// Change the background color for this view.
    pub fn set_background_color(&self, color: Color) {
        {
            let mut inner_self = self.inner_self.borrow_mut();

            if inner_self.background_color == color {
                return;
            }

            inner_self.background_color = color;
        }

        self.set_needs_display();
    }

    /// Request for this view to be redrawn soon.
    ///
    /// See `#draw`, which includes the instructions on what would actually be
    /// drawn to screen.
    pub fn set_needs_display(&self) {
        let behavior = self.behavior.borrow();
        behavior.set_needs_display();
    }

    /// Sets whether this view can be interacted with by the user. If `false`,
    /// then this view will not receive any touch events.
    pub fn set_user_interaction_enabled(&self, enabled: bool) {
        {
            let mut inner_self = self.inner_self.borrow_mut();
            inner_self.user_interaction_enabled = enabled;
        }
    }

    pub fn set_hidden(&self, value: bool) {
        {
            let mut inner_self = self.inner_self.borrow_mut();

            if inner_self.hidden == value {
                return;
            }

            inner_self.hidden = value;
        }

        self.set_needs_display();
    }

    pub fn is_hidden(&self) -> bool {
        self.inner_self.borrow().hidden
    }

    pub fn touches_began(&self, touches: &Vec<Touch>, _event: &TouchEvent) {
        let behavior = self.behavior.borrow();
        behavior.touches_began(touches);
    }

    pub fn touches_ended(&self, touches: &Vec<Touch>, _event: &TouchEvent) {
        let behavior = self.behavior.borrow();
        behavior.touches_ended(touches);
    }

    pub fn touches_moved(&self, touches: &Vec<Touch>, _event: &TouchEvent) {
        let behavior = self.behavior.borrow();
        behavior.touches_moved(touches);
    }

    /// Returns the location of this view in the highest superview coordinate
    /// space (usually the window).
    pub fn get_location_in_window(&self) -> Point<i32> {
        let inner_self = self.inner_self.borrow();
        let superview = inner_self.superview.upgrade();

        if superview.is_none() {
            return inner_self.frame.origin.clone();
        }

        let superview = superview.unwrap();

        let superview_location = superview.get_location_in_window();

        let mut location = inner_self.frame.origin.clone();
        location.x += superview_location.x;
        location.y += superview_location.y;

        location
    }

    /// Convert the given point from the coordinate system of this view to the
    /// coordinate system of the given view.
    pub fn convert_point_to(&self, point: &Point<i32>, to_view: &View) -> Point<i32> {
        let from = self.get_location_in_window();
        let to = to_view.get_location_in_window();

        let x_shift = from.x - to.x;
        let y_shift = from.y - to.y;

        let x = point.x + x_shift;
        let y = point.y + y_shift;

        Point { x, y }
    }

    /// Returns the deepest subview that contains the given point.
    ///
    /// Used for click/touch handling in regards to determining which view it
    /// should fire an event to.
    ///
    /// Will not return views that have `user_interaction_enabled` set to
    /// `false`.
    pub fn hit_test(&self, point: &Point<i32>) -> Option<View> {
        let inner_self = self.inner_self.borrow();

        if inner_self.hidden {
            return None;
        }

        let user_interaction_enabled = inner_self.user_interaction_enabled;

        if inner_self.bounds.contains(point) && user_interaction_enabled {
            for subview in self.subviews().iter().rev() {
                let subview_point = self.convert_point_to(point, subview);
                let hit_test_result = subview.hit_test(&subview_point);

                if hit_test_result.is_some() {
                    return hit_test_result;
                }
            }

            return Some(self.clone());
        }

        None
    }

    pub fn set_frame(&self, frame: Rectangle<i32, u32>) {
        {
            let mut inner_self = self.inner_self.borrow_mut();
            inner_self.frame = frame;
        }

        self.set_needs_display();
    }

    pub fn bounds(&self) -> Rectangle<i32, u32> {
        self.inner_self.borrow().bounds.clone()
    }

    pub fn set_bounds(&self, bounds: Rectangle<i32, u32>) {
        {
            let mut inner_self = self.inner_self.borrow_mut();
            inner_self.bounds = bounds;
        }

        self.set_needs_display();
    }

    /// Returns a boolean indicating whether the given point is contained in
    /// this view's bounds.
    pub fn is_point_inside(&self, point: &Point<i32>) -> bool {
        let inner_self = self.inner_self.borrow();
        let bounds = &inner_self.bounds;
        bounds.contains(point)
    }

    pub fn is_window(&self) -> bool {
        let behavior = self.behavior.borrow();
        behavior.is_window()
    }

    pub fn frame(&self) -> Rectangle<i32, u32> {
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
            behavior: weak_behavior,
            debug_name: self.debug_name.clone()
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

    pub fn subviews(&self) -> Ref<'_, Vec<View>> {
        Ref::map(self.inner_self.borrow(), |inner_self| &inner_self.subviews)
    }
}

impl LayerDelegate for View {
    fn layer_will_draw(&self, _layer: &Layer) {

    }

    fn draw_layer(&self, _layer: &Layer) {
        self.draw();
    }
}

impl Clone for View {
    fn clone(&self) -> Self {
      View {
          id: self.id.clone(),
          inner_self: self.inner_self.clone(),
          behavior: self.behavior.clone(),
          debug_name: self.debug_name.clone()
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
    use crate::graphics::Size;

    #[test]
    fn test_get_location_in_window() {
        let main = View::new(Rectangle {
            origin: Point { x: 0, y: 0 },
            size: Size { width: 100, height: 100 }
        });

        let a = View::new(Rectangle {
            origin: Point { x: 0, y: 1 },
            size: Size { width: 10, height: 10 }
        });

        let b = View::new(Rectangle {
            origin: Point { x: 1, y: 0 },
            size: Size { width: 10, height: 10 }
        });

        let c = View::new(Rectangle {
            origin: Point { x: 1, y: 0 },
            size: Size { width: 10, height: 10 }
        });

        main.add_subview(a.clone());
        a.add_subview(b.clone());
        b.add_subview(c.clone());

        assert_eq!(a.get_location_in_window(), Point { x: 0 as i32, y: 1 as i32 });
        assert_eq!(b.get_location_in_window(), Point { x: 1 as i32, y: 1 as i32 });
        assert_eq!(c.get_location_in_window(), Point { x: 2 as i32, y: 1 as i32 });
    }

    #[test]
    fn test_convert_point_to() {
        let main = View::new(Rectangle {
            origin: Point { x: 0, y: 0 },
            size: Size { width: 100, height: 100 },
        });

        let a = View::new(Rectangle {
            origin: Point { x: 0, y: 1 },
            size: Size { width: 10, height: 10 }
        });

        let b = View::new(Rectangle {
            origin: Point { x: 1, y: 0 },
            size: Size { width: 10, height: 10 }
        });

        let c = View::new(Rectangle {
            origin: Point { x: 1, y: 0 },
            size: Size { width: 10, height: 10 }
        });

        main.add_subview(a.clone());
        a.add_subview(b.clone());
        b.add_subview(c.clone());

        assert_eq!(a.convert_point_to(&Point { x: 0, y: 0 }, &main), Point { x: 0, y: 1 });
        assert_eq!(b.convert_point_to(&Point { x: 0, y: 0 }, &main), Point { x: 1, y: 1 });
        assert_eq!(c.convert_point_to(&Point { x: 0, y: 0 }, &main), Point { x: 2, y: 1 });

        assert_eq!(main.convert_point_to(&Point { x: 2, y: 2 }, &c), Point { x: 0, y: 1 });
    }

    #[test]
    fn test_point_inside() {
        let frame = Rectangle::new(0, 0, 1000, 1000);
        let parent_view = View::new(frame);

        // Red view in the top left
        let frame = Rectangle::new(10, 10, 100, 100);
        let red = View::new(frame);
        red.set_background_color(Color::red());
        parent_view.add_subview(red.clone());

        let point = Point { x: 5, y: 5 };
        let point = parent_view.convert_point_to(&point, &red);
        assert!(!red.is_point_inside(&point));

        let point = Point { x: 50, y: 50 };
        let point = parent_view.convert_point_to(&point, &red);
        assert!(red.is_point_inside(&point));
    }

    #[test]
    fn test_hit_test() {
        let frame = Rectangle::new(0, 0, 1000, 1000);
        let parent_view = View::new(frame);

        // Red view in the top left
        let frame = Rectangle::new(10, 10, 100, 100);
        let red = View::new(frame);
        red.set_background_color(Color::red());
        parent_view.add_subview(red.clone());

        let point = Point { x: 5, y: 5 };
        let result = parent_view.hit_test(&point).unwrap();
        assert_eq!(result, parent_view);

        let point = Point { x: 50, y: 50 };
        let result = parent_view.hit_test(&point).unwrap();
        assert_eq!(result, red);
    }

    #[test]
    fn test_set_frame() {
        let frame = Rectangle::new(0, 0, 1000, 1000);
        let view = View::new(frame);

        let new_frame = Rectangle::new(10, 10, 100, 100);
        view.set_frame(new_frame.clone());

        assert_eq!(view.frame(), new_frame);
    }

    #[test]
    fn test_subviews() {
        let frame = Rectangle::new(0, 0, 1000, 1000);
        let view = View::new(frame);

        let subview = View::new(Rectangle::new(0, 0, 100, 100));
        view.add_subview(subview.clone());

        assert_eq!(view.subviews().len(), 1);
        assert_eq!(view.subviews().get(0).unwrap(), &subview);

        subview.remove_from_superview();
        assert_eq!(view.subviews().len(), 0);
    }

    #[test]
    fn test_bounds() {
        let frame = Rectangle::new(100, 100, 500, 500);
        let view = View::new(frame);

        // By default the same size as the frame, with 0, 0 origin.
        assert_eq!(view.bounds(), Rectangle::new(0, 0, 500, 500));

        view.set_bounds(Rectangle::new(10, 10, 100, 100));

        assert_eq!(view.bounds(), Rectangle::new(10, 10, 100, 100));
    }
}

// use crate::graphics::Context;
use crate::graphics::Rectangle;
use crate::ui::View;
use crate::ui::view::Child;
// use crate::ui::Color;
use crate::ui::view::ViewBehavior;

pub struct Window {
    view: View
}

impl ViewBehavior for Window {
    fn data(&self) -> View {
        self.view.clone()
    }

    fn draw(&self) {

    }
}

impl Window {
    fn new(frame: Rectangle) -> Window {
        let view = View::new(frame);
        Window { view }
    }

//     fn make_key_and_visible(&self) {
//         // make_key
//         self.view.set_hidden(false);
//     }


// }

// impl ViewBehavior for Window {

}

impl PartialEq for Window {
    fn eq(&self, rhs: &Window) -> bool {
        self.view.id == rhs.view.id
    }
}

impl std::fmt::Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.view.id.to_string();

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
    /// Tests add_subview and superview
    fn parent_child_relationship() {
        let frame = Rectangle {
            position: Point { x: 10, y: 10 },
            size: Size { width: 50, height: 50 }
        };

        let mut view_parent = Window::new(frame.clone());
        let view_child = View::new(frame.clone());

        view_parent.add_subview(Child { view: Box::new(view_child.clone()) });

        let view_child1 = view_child.clone();
        let child_inner_self = &view_child1.inner_self.borrow();
        let childs_parent = child_inner_self.superview.as_ref();
        let childs_parent = &childs_parent.unwrap().view;
        let childs_parent_view = childs_parent.upgrade().unwrap().data();

        assert_eq!(view_parent.view.id, childs_parent_view.id);

        let inner_self = view_parent.view.inner_self.borrow();
        let contains_child = inner_self.subviews.contains(&Child { view: Box::new(view_child) });
        assert_eq!(contains_child, true);
    }
}

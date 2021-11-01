use crate::graphics::Point;
use crate::graphics::Size;
use crate::graphics::Rectangle;
use crate::ui::View;
use crate::ui::view::ViewBehavior;

pub trait ViewControllerInner {
    fn new(view: View) -> Self;
    fn view_will_disappear(&self);
    fn view_did_disappear(&self);
    fn view_will_appear(&self);
    fn view_did_appear(&self);
    fn view_did_load(&self);
}

pub struct ViewController<T> {
    view_controller_inner: T
}

impl<T> ViewController<T> where T: ViewControllerInner {
    fn new() -> ViewController<T> {
        let position = Point { x: 100, y: 100 };
        let size = Size { width: 150, height: 150 };
        let frame = Rectangle { position, size };

        let mut view = View::new(frame);


        let view_controller_inner = T::new(view);

        let view_controller = ViewController {
            view_controller_inner
        };

        view_controller
    }
}

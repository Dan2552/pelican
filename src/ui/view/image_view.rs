use crate::graphics::{Image, Rectangle, Point};
use crate::ui::view::DefaultBehavior;
use std::cell::RefCell;
use crate::macros::*;

custom_view!(
    ImageView subclasses DefaultBehavior

    struct ImageViewBehavior {
        image: RefCell<Image<'static>>
    }

    impl Self {
        pub fn new(image: Image<'static>, origin: Point<i32>) -> ImageView {
            let size = image.size().clone();
            let frame = Rectangle { origin, size };
            Self::new_all(frame, RefCell::new(image))
        }
    }

    impl Behavior {
        fn draw(&self) {
            let view = self.view.upgrade().unwrap().clone();
            let inner_self = view.inner_self.borrow();
            let behavior = view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<ImageViewBehavior>().unwrap();

            if let Some(layer) = &inner_self.layer {
                let mut image = behavior.image.borrow_mut();
                let child_layer = image.layer_for(layer.context());
                let rectangle = Rectangle {
                    origin: Point::new(0, 0),
                    size: view.frame().size
                };
                layer.draw_child_layer(&child_layer, &rectangle);
            }
        }
    }
);

use crate::graphics::{Image, Rectangle, Point};
use crate::ui::view::{Behavior, DefaultBehavior};
use std::cell::RefCell;
use crate::macros::*;

custom_view!(
    ImageView subclasses DefaultBehavior 
    
    struct ImageViewBehavior {
        image: RefCell<Image<'static>>
    } 

    view impl {
        pub fn new(image: Image<'static>, origin: Point<i32>) -> ImageView {
            let size = image.size().clone();
            let frame = Rectangle { origin, size };
            Self::new_all(frame, RefCell::new(image))
        }
    }
    
    behavior impl {
        fn draw(&self) {
            // TODO: if image is @2x, scale differently
            let view = self.view.upgrade().unwrap().clone();
            let inner_self = view.inner_self.borrow();
            let behavior = view.behavior.borrow();
            let behavior = behavior.as_any().downcast_ref::<ImageViewBehavior>().unwrap();

            if let Some(layer) = &inner_self.layer {
                let mut image = behavior.image.borrow_mut();
                let child_layer = image.layer_for(layer.context.clone());
                layer.draw_child_layer(&child_layer, &inner_self.frame);
            }
        }
    }
);

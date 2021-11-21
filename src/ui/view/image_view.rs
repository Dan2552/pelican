use crate::graphics::{Image, Rectangle, Point};
use crate::ui::{View, WeakView};
use crate::ui::view::{Behavior, DefaultBehavior};
use std::cell::RefCell;

pub struct ImageViewBehavior {
    view: WeakView,
    super_behavior: Box<dyn Behavior>,
    image: RefCell<Image<'static>>
}

pub struct ImageView {
    pub view: View
}
impl ImageView {
    pub fn new(image: Image<'static>, position: Point<i32>) -> ImageView {
        let size = image.size().clone();
        let super_behavior = DefaultBehavior {
            view: WeakView::none()
        };

        let behavior = ImageViewBehavior { 
            view: WeakView::none(), 
            super_behavior: Box::new(super_behavior), 
            image: RefCell::new(image)
        };

        let frame = Rectangle { position, size };
        let view = View::new_with_behavior(Box::new(behavior), frame);
        ImageView { view }
    }
}

impl Behavior for ImageViewBehavior {
    fn set_view(&mut self, view: WeakView) {
        self.view = view;
    }

    fn get_view(&self) -> &WeakView {
        &self.view
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
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

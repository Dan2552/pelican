use crate::ui::view::View;
use crate::ui::window::WindowBehavior;
use crate::graphics::Layer;
use crate::graphics::Rectangle;
use crate::ui::Window;
use crate::graphics::Context;

pub(crate) fn window_display(window_view: View) {
    if window_view.is_hidden() {
        return;
    }

    let window = Window::from_view(window_view.clone());

    // Additional reference for view controller notification.
    let window1 = window_view.clone();

    let behavior = window_view.behavior.borrow();
    let behavior = behavior.as_any().downcast_ref::<WindowBehavior>().expect("view is not a Window");

    // Recursively draw the texture for each layer that needs redisplay.
    draw_view(&window_view, behavior, &window.context());

    let inner_view = window_view.inner_self.borrow();

    // draw_view lazily creates the layer, so it must exist here.
    let layer = inner_view.layer.as_ref().expect("window layer missing after draw_view");

    // Draw window texture to renderer
    layer.draw_into_context();

    // Actually draw the window to the screen.
    layer.context().draw();

    behavior.view_controller.window_displayed(window1);
}

fn draw_view(view: &View, behavior: &WindowBehavior, context: &Context) {
    let hidden = view.is_hidden();

    {
        {
            let mut inner_view = view.inner_self.borrow_mut();

            // TODO: lazily recreate layer if mismatch contexts
            if inner_view.layer.as_ref().is_none() {
                let size = if inner_view.clips_to_bounds {
                    inner_view.bounds.size.clone()
                } else {
                    inner_view.frame.size.clone()
                };
                let layer = Layer::new(context.clone(), size, Box::new(view.clone()));
                inner_view.layer = Some(layer);
            }

            // Recreate layer if clips_to_bounds and bounds size changed
            if inner_view.clips_to_bounds {
                if let Some(ref layer) = inner_view.layer {
                    if *layer.size() != inner_view.bounds.size {
                        let size = inner_view.bounds.size.clone();
                        let layer = Layer::new(context.clone(), size, Box::new(view.clone()));
                        inner_view.layer = Some(layer);
                    }
                }
            }

            let layer = inner_view.layer.as_mut().expect("layer missing after creation");

            if hidden {
                layer.skip_draw();
                return;
            }

            if !layer.get_needs_display() {
                return;
            }
        }

        let inner_view = view.inner_self.borrow();
        let layer = inner_view.layer.as_ref().expect("layer missing");

        layer.draw();
    }

    let inner_view = view.inner_self.borrow();
    let layer = inner_view.layer.as_ref().expect("layer missing");
    let clips = inner_view.clips_to_bounds;
    let bounds = view.bounds();

    for subview in view.subviews().iter() {
        if subview.is_hidden() {
            continue;
        }

        // Skip subviews entirely outside the visible bounds
        if clips {
            let visible_rect = Rectangle::new(
                bounds.origin.x,
                bounds.origin.y,
                bounds.size.width,
                bounds.size.height,
            );
            let sub_frame = subview.frame();
            if !visible_rect.intersects(&sub_frame) {
                continue;
            }
        }

        // redraw the subview (if it needs it!)
        draw_view(subview, behavior, context);

        let sub_inner_view = subview.inner_self.borrow();
        let subview_layer = match sub_inner_view.layer.as_ref() {
            Some(l) => l,
            None => continue,
        };

        let frame = subview.frame();

        // For clips_to_bounds children, the texture is bounds-sized not
        // frame-sized, so use bounds.size for the destination dimensions.
        let (dest_width, dest_height) = if sub_inner_view.clips_to_bounds {
            (sub_inner_view.bounds.size.width, sub_inner_view.bounds.size.height)
        } else {
            (frame.size.width, frame.size.height)
        };

        let frame_relative_to_superview_bounds = Rectangle::new(
            frame.origin.x - bounds.origin.x,
            frame.origin.y - bounds.origin.y,
            dest_width,
            dest_height,
        );

        layer.draw_child_layer(subview_layer, &frame_relative_to_superview_bounds);
    }
}

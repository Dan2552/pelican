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
    let behavior = behavior.as_any().downcast_ref::<WindowBehavior>().unwrap();

    // Recursively draw the texture for each layer that needs redisplay.
    draw_view(&window_view, behavior, &window.context());

    let inner_view = window_view.inner_self.borrow();

    // If layer was not present before this function was invoked, the leading
    // `draw_view` will have lazily created the layer, so we can be certain it
    // can be `unwrapped` here.
    let layer = inner_view.layer.as_ref().unwrap();

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
                let size = inner_view.frame.size.clone();
                let layer = Layer::new(context.clone(), size, Box::new(view.clone()));
                inner_view.layer = Some(layer);
            }

            let layer = inner_view.layer.as_mut().unwrap();

            if hidden {
                layer.skip_draw();
                return;
            }

            if !layer.get_needs_display() {
                return;
            }
        }

        let inner_view = view.inner_self.borrow();
        let layer = inner_view.layer.as_ref().unwrap();

        layer.draw();
    }

    let inner_view = view.inner_self.borrow();
    let layer = inner_view.layer.as_ref().unwrap();

    for subview in view.subviews().iter() {
        // redraw the subview (if it needs it!)
        draw_view(subview, behavior, context);

        if subview.is_hidden() {
            continue;
        }

        let sub_inner_view = subview.inner_self.borrow();
        let subview_layer = sub_inner_view.layer.as_ref().unwrap();

        let frame = subview.frame();

        let frame_relative_to_superview_bounds = Rectangle::new(
            frame.origin.x - view.bounds().origin.x,
            frame.origin.y - view.bounds().origin.y,
            frame.size.width,
            frame.size.height,
        );

        layer.draw_child_layer(subview_layer, &frame_relative_to_superview_bounds);
    }
}

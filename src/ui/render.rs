use crate::ui::view::View;
use crate::ui::window::WindowBehavior;
use crate::graphics::{Rectangle, Point, Size, Layer};

pub(crate) fn window_display(window: View) {
    let inner_view = window.inner_self.borrow();
    let behavior = window.behavior.borrow();
    let behavior = behavior.as_any().downcast_ref::<WindowBehavior>().unwrap();

    // Recursively draw the texture for each layer that needs redisplay.
    draw_view(&window, behavior);

    // If layer was not present before this function was invoked, the leading
    // `draw_view` will have lazily created the layer, so we can be certain it
    // can be `unwrapped` here.
    let layer = inner_view.layer.as_ref().unwrap();

    // Draw window texture to renderer
    layer.draw_into_context();

    // Actually draw the window to the screen.
    layer.context.draw();
}

fn draw_view(view: &View, behavior: &WindowBehavior) {
    {
        let mut inner_view = view.inner_self.borrow_mut();

        // TODO: lazily recreate layer if mismatch contexts
        if inner_view.layer.as_ref().is_none() {
            let context = behavior.graphics_context.clone();
            let size = inner_view.frame.size.clone();
            let layer = Layer::new(context, size, Box::new(view.clone()));
            inner_view.layer = Some(layer);
        }

        let layer = inner_view.layer.as_mut().unwrap();

        if layer.get_needs_display() {
            return;
        }

        layer.draw();
    }

    let inner_view = view.inner_self.borrow();
    let layer = inner_view.layer.as_ref().unwrap();

    for subview in &inner_view.subviews {
        // redraw the subview (if it needs it!)
        draw_view(subview, behavior);

        let x_offset = 0;
        let y_offset = 0;

        // TODO: content offset. Though maybe better handled through view
        // bounds?
        // if view.respond_to?(:content_offset)
        //   x_offset = view.content_offset.x
        //   y_offset = view.content_offset.y
        // end
        let sub_inner_view = subview.inner_self.borrow();
        let subview_layer = sub_inner_view.layer.as_ref().unwrap();

        let frame = subview.get_frame();

        let render_scale = layer.context.render_scale;
        let x = ((frame.position.x + x_offset) as f32 * render_scale).round() as i32;
        let y = ((frame.position.y + y_offset) as f32 * render_scale).round() as i32;
        let width = (frame.size.width as f32 * render_scale).round() as u32;
        let height = (frame.size.height as f32 * render_scale).round() as u32;

        let destination = Rectangle {
            position: Point { x, y },
            size: Size { width, height }
        };

        layer.draw_child_layer(subview_layer, &destination);
    }
}

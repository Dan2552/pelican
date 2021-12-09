use crate::graphics::Rectangle;
use crate::graphics::Size;
use crate::ui::view::View;
use crate::ui::Color;
use crate::ui::view::DefaultBehavior;
use crate::macros::*;

custom_view!(
    ScrollView subclasses DefaultBehavior

    struct ScrollViewBehavior {

    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>) -> Self {
            let scroll_bar_frame = Rectangle::new(
                frame.size.width as i32 - 10,
                0,
                10,
                frame.size.height
            );

            let vertical_scroll_bar = ScrollBarView::new(scroll_bar_frame);

            let scroll_bar_frame = Rectangle::new(
                0,
                frame.size.height as i32 - 10,
                frame.size.width,
                10
            );

            let horizontal_scroll_bar = ScrollBarView::new(scroll_bar_frame);

            let content_view = View::new(Rectangle::new(0, 0, 0, 0));
            content_view.set_background_color(Color::clear());

            let scroll_view = Self::new_all(frame);
            scroll_view.view.set_background_color(Color::clear());
            scroll_view.view.add_subview(content_view);
            scroll_view.view.add_subview(vertical_scroll_bar.view);
            scroll_view.view.add_subview(horizontal_scroll_bar.view);

            scroll_view
        }

        fn inner_content_view(&self) -> View {
            self.view.subviews().get(0).unwrap().clone()
        }

        /// Set (or replace) the current content view. This is the view that
        /// will actually be scrollable.
        pub fn set_content_view(&self, view: View) {
            if let Some(existing_subview) = self.inner_content_view().subviews().get(0) {
                existing_subview.remove_from_superview();
            }

            self.update_content_size(view.frame().size);

            self.inner_content_view().add_subview(view);
        }

        /// Get the current content view, if there is one.
        pub fn content_view(&self) -> Option<View> {
            if let Some(content_view) = self.inner_content_view().subviews().get(0) {
                Some(content_view.clone())
            } else {
                None
            }
        }

        fn vertical_scroll_bar(&self) -> ScrollBarView {
            let view = self.view.subviews().get(1).unwrap().clone();
            ScrollBarView::from_view(view)
        }

        fn horizontal_scroll_bar(&self) -> ScrollBarView {
            let view = self.view.subviews().get(2).unwrap().clone();
            ScrollBarView::from_view(view)
        }

        fn update_content_size(&self, size: Size<u32>) {
            let inner_content_view = self.inner_content_view();
            inner_content_view.set_frame(Rectangle::new(0, 0, size.width, size.height));

            let vertical_scroll_bar_handle = self.vertical_scroll_bar().handle();
            let horizontal_scroll_bar_handle = self.horizontal_scroll_bar().handle();

            let scrollview_height = self.view.frame().size.height;
            let content_view_height = size.height;
            let height_of_vertical_handle = ((scrollview_height as f32 / content_view_height as f32 ) * scrollview_height as f32) as u32;

            vertical_scroll_bar_handle.set_frame(
                Rectangle {
                    origin: vertical_scroll_bar_handle.frame().origin,
                    size: Size { width: 10, height: height_of_vertical_handle }
                }
            );

            let scrollview_width = self.view.frame().size.width;
            let content_view_width = size.width;
            let width_of_horizontal_handle = ((scrollview_width as f32 / content_view_width as f32 ) * scrollview_width as f32) as u32;

            horizontal_scroll_bar_handle.set_frame(
                Rectangle {
                    origin: horizontal_scroll_bar_handle.frame().origin,
                    size: Size { width: width_of_horizontal_handle, height: 10 }
                }
            );

            if scrollview_width == width_of_horizontal_handle {
                horizontal_scroll_bar_handle.set_hidden(true);
            } else {
                horizontal_scroll_bar_handle.set_hidden(false);
            }

            if scrollview_height == height_of_vertical_handle {
                vertical_scroll_bar_handle.set_hidden(true);
            } else {
                vertical_scroll_bar_handle.set_hidden(false);
            }
        }
    }
);

custom_view!(
    ScrollBarView subclasses DefaultBehavior

    struct ScrollBarViewBehavior {}

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>) -> Self {
            let handle_frame = Rectangle::new(0, 0, 10, 10);
            let handle = View::new(handle_frame);

            handle.set_background_color(Color::new(127, 127, 127, 127));

            let scroll_bar_view = Self::new_all(frame);
            scroll_bar_view.view.set_background_color(Color::clear());
            scroll_bar_view.view.add_subview(handle);

            scroll_bar_view
        }

        fn handle(&self) -> View {
            self.view.subviews().get(0).unwrap().clone()
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Size;

    #[test]
    fn test_vertical_scroll_bar_size() {
        let scroll_view = ScrollView::new(Rectangle::new(0, 0, 100, 100));

        assert_eq!(
            scroll_view.vertical_scroll_bar().view.frame().size,
            Size::new(10, 100)
        );

        assert_eq!(
            scroll_view.vertical_scroll_bar().handle().frame().size,
            Size::new(10, 10)
        );
    }

    #[test]
    fn test_horizontal_scroll_bar_size() {
        let scroll_view = ScrollView::new(Rectangle::new(0, 0, 100, 100));

        assert_eq!(
            scroll_view.horizontal_scroll_bar().view.frame().size,
            Size::new(100, 10)
        );

        assert_eq!(
            scroll_view.vertical_scroll_bar().handle().frame().size,
            Size::new(10, 10)
        );
    }
}

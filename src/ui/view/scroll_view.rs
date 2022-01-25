use crate::graphics::Rectangle;
use crate::graphics::Size;
use crate::ui::view::View;
use crate::ui::Color;
use crate::ui::view::DefaultBehavior;
use crate::macros::*;
use crate::ui::gesture::pan_recognizer::PanRecognizer;
use crate::graphics::Point;
use std::cell::Cell;

custom_view!(
    ScrollView subclasses DefaultBehavior

    struct ScrollViewBehavior {

    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>) -> Self {
            let vertical_scroll_bar = ScrollBarView::new(ScrollBarDirection::Vertical);
            let horizontal_scroll_bar = ScrollBarView::new(ScrollBarDirection::Horizontal);

            let content_view = View::new(Rectangle::new(0, 0, 0, 0));
            content_view.set_background_color(Color::clear());

            let scroll_view = Self::new_all(frame);
            scroll_view.view.set_background_color(Color::clear());
            scroll_view.view.add_subview(content_view);
            scroll_view.view.add_subview(vertical_scroll_bar.view.clone());
            scroll_view.view.add_subview(horizontal_scroll_bar.view.clone());

            vertical_scroll_bar.fit_to_superview();
            horizontal_scroll_bar.fit_to_superview();

            let pan_gesture = PanRecognizer::new(|gesture_recognizer| {
                if gesture_recognizer.view().is_none() {
                    return;
                }

                let view = gesture_recognizer.view().upgrade().unwrap();
                let scroll_view = ScrollView::from_view(view.clone());

                let translation = gesture_recognizer.translation_in(&view);

                let translation = Point::new(
                    -translation.x,
                    -translation.y
                );

                scroll_view.set_content_offset(
                    scroll_view.content_offset() + translation
                );

                gesture_recognizer.set_translation(Point::new(0, 0), &view);
            });

            scroll_view.view.add_gesture_recognizer(Box::new(pan_gesture));

            scroll_view
        }

        pub fn content_offset(&self) -> Point<i32> {
            self.inner_content_view().bounds().origin
        }

        fn content_size(&self) -> Size<u32> {
            if let Some(content_view) = self.content_view() {
                content_view.frame().size
            } else {
                Size::new(0, 0)
            }
        }

        fn set_content_offset(&self, offset: Point<i32>) {
            let mut content_width = self.content_size().width;
            let scrollview_width = self.view.frame().size.width;

            let mut content_height = self.content_size().height;
            let scrollview_height = self.view.frame().size.height;

            if content_width < scrollview_width {
                content_width = scrollview_width;
            }

            if content_height < scrollview_height {
                content_height = scrollview_height;
            }

            let max_x = content_width - scrollview_width;
            let max_y = content_height - scrollview_height;

            let x = offset.x.max(0).min(max_x as i32);
            let y = offset.y.max(0).min(max_y as i32);

            self.inner_content_view().set_bounds(
                Rectangle::new(
                    x,
                    y,
                    self.view.bounds().size.width,
                    self.view.bounds().size.height
                )
            );

            let vertical_percent = y as f32 / max_y as f32 * 100.0;
            let horizontal_percent = x as f32 / max_x as f32 * 100.0;

            self.vertical_scroll_bar().set_percent(vertical_percent as u8);
            self.horizontal_scroll_bar().set_percent(horizontal_percent as u8);
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

            self.vertical_scroll_bar().update_scroll_handle();
            self.horizontal_scroll_bar().update_scroll_handle();
        }
    }
);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ScrollBarDirection {
    Vertical,
    Horizontal
}

custom_view!(
    ScrollBarView subclasses DefaultBehavior

    struct ScrollBarViewBehavior {
        direction: ScrollBarDirection,
        percent: Cell<u8>
    }

    impl Self {
        fn new(direction: ScrollBarDirection) -> Self {
            let handle = View::new(Rectangle::new(0, 0, 10, 10));
            handle.set_background_color(Color::new(127, 127, 127, 127));

            let scroll_bar_view = Self::new_all(Rectangle::new(0, 0, 10, 10), direction, Cell::new(0));
            scroll_bar_view.view.set_background_color(Color::clear());
            scroll_bar_view.view.add_subview(handle);

            scroll_bar_view
        }

        fn percent(&self) -> u8 {
            let behavior = self.behavior();
            behavior.percent.get()
        }

        fn set_percent(&self, percent: u8) {
            let behavior = self.behavior();
            behavior.percent.set(percent);
            self.update_scroll_handle();
        }

        fn handle(&self) -> View {
            self.view.subviews().get(0).unwrap().clone()
        }

        fn direction(&self) -> ScrollBarDirection {
            let behavior = self.behavior();
            behavior.direction
        }

        fn fit_to_superview(&self) {
            let superview = self.view.superview().upgrade().unwrap();
            let superview_size = superview.frame().size;
            let frame: Rectangle<i32, u32>;

            match self.direction() {
                ScrollBarDirection::Vertical => {
                    frame = Rectangle::new(
                        superview_size.width as i32 - 10,
                        0,
                        10,
                        superview_size.height
                    );
                },
                ScrollBarDirection::Horizontal => {
                    frame = Rectangle::new(
                        0,
                        superview_size.height as i32 - 10,
                        superview_size.width,
                        10
                    );
                }
            }

            self.view.set_frame(frame);
        }

        fn update_scroll_handle(&self) {
            let superview = self.view.superview().upgrade().unwrap();
            let scrollview = ScrollView::from_view(superview);
            let inner_content_view = scrollview.inner_content_view();
            let handle = self.handle();

            handle.set_hidden(true);

            let content_view_size = inner_content_view.frame().size;
            let scrollview_size = scrollview.view.frame().size;

            match self.direction() {
                ScrollBarDirection::Vertical => {
                    let height_of_vertical_handle: u32;

                    if content_view_size.height == 0 {
                        height_of_vertical_handle = 0;
                    } else {
                        height_of_vertical_handle = (
                            (scrollview_size.height as f32 / content_view_size.height as f32 ) *
                            scrollview_size.height as f32
                        ) as u32;
                    }

                    let handle_size = Size {
                        width: 10,
                        height: height_of_vertical_handle
                    };

                    let percent_of_scrollview = self.percent() as f32 / 100.0;
                    let origin_y = (percent_of_scrollview * (scrollview_size.height - handle_size.height) as f32) as i32;
                    let origin_x = handle.frame().origin.x;

                    handle.set_frame(Rectangle {
                        origin: Point::new(origin_x, origin_y),
                        size: handle_size
                    });

                    handle.set_hidden(scrollview_size.height == height_of_vertical_handle);
                },
                ScrollBarDirection::Horizontal => {
                    let width_of_horizontal_handle: u32;

                    if content_view_size.width == 0 {
                        width_of_horizontal_handle = 0;
                    } else {
                        width_of_horizontal_handle = (
                            (scrollview_size.width as f32 / content_view_size.width as f32 ) *
                            scrollview_size.width as f32
                        ) as u32;
                    }


                    let handle_size = Size {
                        width: width_of_horizontal_handle,
                        height: 10
                    };

                    let percent_of_scrollview = self.percent() as f32 / 100.0;
                    let origin_x = (percent_of_scrollview * (scrollview_size.width - handle_size.width) as f32) as i32;
                    let origin_y = handle.frame().origin.y;

                    handle.set_frame(Rectangle {
                        origin: Point::new(origin_x, origin_y),
                        size: handle_size
                    });

                    handle.set_hidden(scrollview_size.width == width_of_horizontal_handle);
                }
            }
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

    #[test]
    fn test_content_offset() {
        // No content view meaning it's not scrollable
        {
            let scroll_view = ScrollView::new(Rectangle::new(0, 0, 100, 100));
            assert_eq!(scroll_view.content_offset(), Point::new(0, 0));
            scroll_view.set_content_offset(Point::new(10, 10));
            assert_eq!(scroll_view.content_offset(), Point::new(0, 0));
        }

        // With content view of the same size as the scroll view, meaning it's
        // not scrollable still.
        {
            let scroll_view = ScrollView::new(Rectangle::new(0, 0, 100, 100));
            let content_view = View::new(Rectangle::new(0, 0, 100, 100));
            scroll_view.set_content_view(content_view);

            assert_eq!(scroll_view.content_offset(), Point::new(0, 0));
            scroll_view.set_content_offset(Point::new(10, 10));
            assert_eq!(scroll_view.content_offset(), Point::new(0, 0));
        }

        // With content view bigger than the scroll view
        {
            let scroll_view = ScrollView::new(Rectangle::new(0, 0, 100, 100));
            let content_view = View::new(Rectangle::new(0, 0, 200, 200));
            scroll_view.set_content_view(content_view);

            assert_eq!(scroll_view.content_offset(), Point::new(0, 0));
            scroll_view.set_content_offset(Point::new(10, 10));
            assert_eq!(scroll_view.content_offset(), Point::new(10, 10));
        }
    }
}

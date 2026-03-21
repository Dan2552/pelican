use crate::graphics::Rectangle;
use crate::ui::view::DefaultBehavior;
use crate::ui::window::Window;
use crate::ui::Color;
use crate::ui::Touch;
use crate::macros::*;
use wry::raw_window_handle::{AppKitWindowHandle, HandleError, HasWindowHandle, RawWindowHandle, WindowHandle};
use wry::WebViewBuilder;
use std::ptr::NonNull;
use std::ffi::c_void;
use std::cell::{RefCell, Cell};

struct NsContentViewParent {
    ns_view: NonNull<c_void>,
}

impl HasWindowHandle for NsContentViewParent {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let appkit = AppKitWindowHandle::new(self.ns_view);
        let raw = RawWindowHandle::AppKit(appkit);
        // SAFETY: we're creating a borrowed handle from a stable pointer
        Ok(unsafe { WindowHandle::borrow_raw(raw) })
    }
}

pub(crate) enum WebViewContent {
    Url(String),
    Html(String),
}

custom_view!(
    WebView subclasses DefaultBehavior

    struct WebViewBehavior {
        wry_webview: RefCell<Option<wry::WebView>>,
        pending_content: RefCell<Option<WebViewContent>>,
        focused: Cell<bool>
    }

    impl Self {
        pub fn new(frame: Rectangle<i32, u32>) -> Self {
            let web_view = Self::new_all(
                frame,
                RefCell::new(None),
                RefCell::new(None),
                Cell::new(false),
            );
            web_view.set_background_color(Color::clear());
            web_view
        }

        pub fn load_url(&self, url: &str) {
            let behavior = self.behavior();
            let wry = behavior.wry_webview.borrow();
            if let Some(webview) = wry.as_ref() {
                let _ = webview.load_url(url);
            } else {
                drop(wry);
                behavior.pending_content.replace(Some(WebViewContent::Url(url.to_string())));
                self.set_needs_display();
            }
        }

        pub fn load_html(&self, html: &str) {
            let behavior = self.behavior();
            let wry = behavior.wry_webview.borrow();
            if let Some(webview) = wry.as_ref() {
                let _ = webview.load_html(html);
            } else {
                drop(wry);
                behavior.pending_content.replace(Some(WebViewContent::Html(html.to_string())));
                self.set_needs_display();
            }
        }

        fn find_window(&self) -> Option<Window> {
            let mut current_view = self.view.clone();
            loop {
                if current_view.is_window() {
                    return Some(Window::from_view(current_view));
                }
                if let Some(superview) = current_view.superview().upgrade() {
                    current_view = superview;
                } else {
                    return None;
                }
            }
        }

        fn ensure_webview(&self) -> bool {
            {
                let behavior = self.behavior();
                if behavior.wry_webview.borrow().is_some() {
                    return true;
                }
            }

            let window = match self.find_window() {
                Some(w) => w,
                None => return false,
            };

            let ns_view = match window.context().ns_content_view() {
                Some(v) => v,
                None => return false,
            };

            let parent = NsContentViewParent { ns_view };

            // Capture a WeakView so the IPC handler can call become_first_responder
            // directly when the native webview is clicked.
            let weak_view = self.view.downgrade();

            let builder = WebViewBuilder::new()
                .with_transparent(true)
                .with_focused(false)
                .with_initialization_script(
                    "document.addEventListener('mousedown', function() { \
                        window.ipc.postMessage('clicked'); \
                    }, true);"
                )
                .with_ipc_handler(move |_req| {
                    if let Some(view) = weak_view.upgrade() {
                        view.become_first_responder();
                    }
                });

            let webview = match builder.build_as_child(&parent) {
                Ok(wv) => wv,
                Err(_) => return false,
            };

            // Don't let the native webview steal focus on creation
            let _ = webview.focus_parent();

            let behavior = self.behavior();

            // Load pending content
            let pending = behavior.pending_content.borrow_mut().take();
            match pending {
                Some(WebViewContent::Url(url)) => { let _ = webview.load_url(&url); }
                Some(WebViewContent::Html(html)) => { let _ = webview.load_html(&html); }
                None => {}
            }

            behavior.wry_webview.borrow_mut().replace(webview);
            true
        }

        fn sync_bounds(&self) {
            let behavior = self.behavior();
            let wry = behavior.wry_webview.borrow();
            if let Some(webview) = wry.as_ref() {
                let location = self.view.get_location_in_window();
                let frame = self.view.frame();

                let _ = webview.set_bounds(wry::Rect {
                    position: dpi::Position::Logical(dpi::LogicalPosition {
                        x: location.x as f64,
                        y: location.y as f64,
                    }),
                    size: dpi::Size::Logical(dpi::LogicalSize {
                        width: frame.size.width as f64,
                        height: frame.size.height as f64,
                    }),
                });
            }
        }

        fn native_focus(&self) {
            let behavior = self.behavior();
            let wry = behavior.wry_webview.borrow();
            if let Some(webview) = wry.as_ref() {
                let _ = webview.focus();
            }
            behavior.focused.set(true);
        }

        fn native_blur(&self) {
            let behavior = self.behavior();
            let wry = behavior.wry_webview.borrow();
            if let Some(webview) = wry.as_ref() {
                let _ = webview.focus_parent();
            }
            behavior.focused.set(false);
        }

        fn sync_focus_state(&self) {
            let is_focused = self.behavior().focused.get();

            if let Some(window) = self.find_window() {
                let first_responder = window.first_responder();
                let we_are_first_responder = first_responder.id() == self.view.id();

                if we_are_first_responder && !is_focused {
                    self.native_focus();
                } else if !we_are_first_responder && is_focused {
                    self.native_blur();
                }
            }
        }
    }

    impl Behavior {
        fn draw(&self) {
            let view_type = self.view_type();
            view_type.ensure_webview();
            view_type.sync_bounds();
            view_type.sync_focus_state();

            // Clear the layer to transparent so the native webview shows through
            let view = self.view.upgrade().expect("view was deallocated");
            let inner_self = view.inner_self.borrow();
            if let Some(layer) = &inner_self.layer {
                layer.clear_with_color(sdl2::pixels::Color::RGBA(0, 0, 0, 0));
            }
        }

        fn touches_began(&self, _touches: &Vec<Touch>) {
            let view = self.view.upgrade().expect("view was deallocated");
            view.become_first_responder();
        }

        fn handles_native_keyboard_input(&self) -> bool {
            true
        }

        fn did_resign_first_responder(&self) {
            let view_type = self.view_type();
            view_type.native_blur();
        }

        fn did_become_first_responder(&self) {
            // Stop SDL2 text input so it doesn't intercept/duplicate key
            // events that the native WKWebView should handle exclusively.
            unsafe { sdl2::sys::SDL_StopTextInput(); }
            let view_type = self.view_type();
            view_type.native_focus();
        }
    }
);

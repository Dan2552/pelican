use crate::graphics::{Layer, Rectangle};
use crate::ui::Color;
use crate::ui::view::{View, WeakView};
use crate::ui::gesture::recognizer::Recognizer;
use std::rc::Rc;

pub(crate) struct ViewInner {
    /// Some way to compare `View`s (`==`) and `WeakView`s
    pub(crate) id: usize,

    /// An optional identifier for the view. Can be used to find the view in
    /// the view hierarchy.
    ///
    /// See `View::view_with_tag`
    pub(crate) tag: u32,

    /// The size and position (within its superview) of this View.
    ///
    /// Used for placing the view in the parent.
    pub frame: Rectangle<i32, u32>,

    /// The size and position of the View from the view's own coordinate
    /// perspective.
    ///
    /// Will commonly have the same size as `frame`, but in most circumstances
    /// the position will be `0,0`.
    ///
    /// When the position is changed, the internal contents will move rather
    /// than the View itself. For example, this could be used to create behavior
    /// like a scroll view. E.g. if an image were inside this view, it could be
    /// used to pan the image.
    ///
    /// If you still don't get it, see:
    /// https://stackoverflow.com/a/28917673/869367
    pub bounds: Rectangle<i32, u32>,

    /// The background color of the view. In its simplest form, a View is just a
    /// rectangle with a single color - this is that color.
    pub background_color: Color,

    /// The actual drawable canvas from the `graphics` library.
    ///
    /// Think of the View as instructions or a template for a picture (this
    /// behavior itself defined in `#draw`), and then the `layer` is the canvas
    /// that picture is drawn onto.
    ///
    /// The layer will also handle lifecycle of when the view is to be drawn.
    /// That is to say, the layer will call this view (it's `delegate`) to draw
    /// when the platform calls for it to be drawn (the `layer` itself will be
    /// the thing calling `#draw` for this view).
    ///
    /// A layer will only ever be present if this view is contained within a
    /// window in the view heirarchy. It will be replaced with a fresh layer if
    /// the parent view is ever changed.
    ///
    /// `render::window_display()` itself manages the lifecycle of this; it is
    /// not refreshed immediately upon changes to the view heirarchy.
    pub layer: Option<Layer>,

    /// The parent view; the view that contains (and owns) this one.
    pub superview: WeakView,

    /// Children views; views that are contained (and owned) within this view.
    pub subviews: Vec<View>,

    /// Gesture recognizers that are attached to this view.
    pub gesture_recognizers: Vec<Rc<Box<dyn Recognizer>>>,

    /// Whether this view is visible or not. When hidden at the next render to
    /// screen, it'll behave the same as if it were not in the view hierarchy at
    /// all.
    pub hidden: bool,

    /// Whether the view accepts user input or not. E.g. touches_began will not
    /// be called if this is `false`.
    pub user_interaction_enabled: bool
}

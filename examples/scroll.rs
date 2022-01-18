use pelican::graphics::Rectangle;
use pelican::ui::{View, Window, Color};
use pelican::ui::{ApplicationMain, ApplicationDelegate};
use pelican::ui::{ViewController, ViewControllerBehavior};
use pelican::ui::ScrollView;
use pelican::ui::button::Button;
use objc_foundation::{NSArray, NSDictionary, NSObject, NSString,
    INSArray, INSCopying, INSDictionary, INSObject, INSString};
use objc::runtime::{BOOL, Class, Object, Sel, YES};
use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use objc::rc::StrongPtr;
use objc_id::{Id, WeakId};

struct ExampleViewController {}
impl ViewControllerBehavior for ExampleViewController {
    fn view_did_load(&self, view: View) {
        // Setup the scrollview the same size as the window.
        let frame = Rectangle::new(0, 0, 400, 200);
        let scroll_view = ScrollView::new(frame);

        // View as "content" for the scrollview.
        let content_view = View::new(Rectangle::new(0, 0, 800, 400));

        let frame = Rectangle::new(0, 50, 400, 400 - 100);
        let child = View::new(frame);
        child.set_background_color(Color::gray());
        content_view.add_subview(child);

        let frame = Rectangle::new(0, 100, 100, 30);
        let button = Button::new(frame, "Button", move || {
            println!("button tapped");
        });
        button.view.set_background_color(Color::white());
        content_view.add_subview(button.view);

        scroll_view.set_content_view(content_view);
        view.add_subview(scroll_view.view);
    }
}

struct AppDelegate {}
impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self) {
        let frame = Rectangle::new(200, 200, 400, 200);
        let view_controller = ViewController::new(ExampleViewController {});
        let window = Window::new("Scroll example", frame, view_controller);
        window.make_key_and_visible();
    }
}


// unsafe fn from_refs<D, T>(keys: &[&T], vals: &[&D::Value]) -> Id<D>
//         where D: INSDictionary, T: INSCopying<Output=D::Key> {
//     let cls = D::class();
//     let count = min(keys.len(), vals.len());
//     let obj: *mut D = msg_send![cls, alloc];
//     let obj: *mut D = msg_send![obj, initWithObjects:vals.as_ptr()
//                                              forKeys:keys.as_ptr()
//                                                count:count];
//     Id::from_retained_ptr(obj)
// }

pub fn main() -> Result<(), String> {

    // NSDictionary *appDefaults = [[NSDictionary alloc] initWithObjectsAndKeys:
    // [NSNumber numberWithBool:YES], @"AppleMomentumScrollSupported",
    // nil];
    // [[NSUserDefaults standardUserDefaults] registerDefaults:appDefaults];

    let obj: Id<Object> = unsafe {
        let obj: *mut Object = msg_send![class!(NSNumber), numberWithBool:YES];
        Id::from_retained_ptr(obj)
    };

    let string = NSString::from_str("AppleMomentumScrollSupported");
    let keys = &[&*string];
    let vals = vec![obj];
    // let dict = NSDictionary::from_keys_and_objects(keys, vals);

    let app_defaults: Id<Object> = unsafe {
        let obj: *mut Object = msg_send![class!(NSDictionary), alloc];
        // let obj: *mut Object = msg_send![obj, initWithObjects:vals.as_ptr()
        //                                       forKeys:keys.as_ptr()
        //                                       count:1];

        let obj: *mut Object = msg_send![obj, initWithObjects:vals.as_ptr()
                                              forKeys:(&[&*string]).as_ptr()
                                              count:1];
        Id::from_retained_ptr(obj)
    };

    unsafe {
        let obj: *mut Object = msg_send![class!(NSUserDefaults), standardUserDefaults];
        let obj: *mut Object = msg_send![obj, registerDefaults:app_defaults];
    }

    let app_delegate = AppDelegate {};
    let application_main = ApplicationMain::new(Box::new(app_delegate));
    application_main.launch();
    Ok(())
}

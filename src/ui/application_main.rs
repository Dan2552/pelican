use crate::ui::{RunLoop, Timer};
use crate::ui::event_loop;
use std::time::Duration;

use objc_foundation::{NSString,  INSString};
use objc::runtime::{Object, YES};
use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;

pub trait ApplicationDelegate {
    fn application_will_finish_launching(&self) {}
    fn application_did_finish_launching(&self) {}
    fn application_did_become_active(&self) {}
    fn application_will_terminate(&self) {}
}

pub struct ApplicationMain {
    delegate: Box<dyn ApplicationDelegate>
}

impl ApplicationMain {
    pub fn new(delegate: Box<dyn ApplicationDelegate>) -> ApplicationMain {
        ApplicationMain {
            delegate: delegate
        }
    }

    pub fn launch(self) {
        if cfg!(target_os = "macos") {
            unsafe {
                let key = NSString::from_str("AppleMomentumScrollSupported");
                let obj: *mut Object = msg_send![class!(NSUserDefaults), standardUserDefaults];
                let _: *mut Object = msg_send![obj, setBool:YES forKey:key];
            }
        }

        self.delegate.application_will_finish_launching();
        self.delegate.application_did_finish_launching();

        // Startup the RunLoop with the event loop as the only process to run.
        // Upon needing it, the UI code will add timers to deal with rendering
        // specific parts. This means the RunLoop won't needlessly iterate the
        // whole view tree every loop in order to work out if something needs
        // re-rendering.
        let run_loop = RunLoop::borrow();

        // The only default timer that is started by defualt is the event loop.
        // The event loop will handle all OS events; any user or device input
        // and propagate to the appropriate areas of the application.
        {
            let sdl: &sdl2::Sdl;
            unsafe { sdl = crate::graphics::SDL_CONTAINER.lazy(); }

            let duration = Duration::from_millis(0);
            let timer = Timer::new_repeating(duration, move || event_loop::update(sdl));
            run_loop.add_timer(timer);
        }

        self.delegate.application_did_become_active();
        run_loop.run();

        self.delegate.application_will_terminate();
    }
}

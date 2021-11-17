use crate::ui::Application;

pub trait ApplicationDelegate {
    fn application_will_finish_launching(&self) {}
    fn application_did_finish_launching(&self) {}
    fn application_did_become_active(&self) {}
    fn application_will_terminate(&self) {}
}

pub(crate) struct ApplicationMain {
    delegate: Box<dyn ApplicationDelegate>
}

impl ApplicationMain {
    fn new(delegate: Box<dyn ApplicationDelegate>) -> ApplicationMain {
        ApplicationMain {
            delegate: delegate
        }
    }

    fn launch(self) {
        self.delegate.application_will_finish_launching();
        self.delegate.application_did_finish_launching();

        // Startup the RunLoop with the event loop as the only process to run.
        // Upon needing it, the UI code will add timers to deal with rendering
        // specific parts. This means the RunLoop won't needlessly iterate the
        // whole view tree every loop in order to work out if something needs
        // re-rendering.
        // TODO:
        // run_loop = RunLoop.main
        // event_loop = Timer.new(
        //   repeats: true,
        //   object: EventLoop.new,
        //   selector: :update
        // )
        // run_loop.add_timer(event_loop)

        self.delegate.application_did_become_active();

        // TODO:
        // run_loop.run

        self.delegate.application_will_terminate();
    }
}

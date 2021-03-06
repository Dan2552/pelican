use crate::ui::timer::Timer;
use std::time::Instant;
use std::thread::sleep;
use std::cell::{Cell, RefCell};
use std::time::Duration;
use crate::macros::*;
use crate::platform::thread;

singleton!(
    RunLoop,
    timers: RefCell::new(Vec::new()),
    state: Cell::new(State::Running)
);

pub struct RunLoop {
    timers: RefCell<Vec<Timer>>,
    state: Cell<State>
}

impl RunLoop {
    pub fn add_timer(&self, timer: Timer) {
        if !thread::is_main() {
            println!("Warning: attempted to add timer from non-main thread. The timer has not been added.");
            return;
        }
        let mut timers = self.timers.borrow_mut();
        timers.push(timer)
    }

    /// Run the run loop until the application exits.
    ///
    /// This isn't intended to be called in your app.
    pub fn run(&self) {
        let mut last_loop_instant = Instant::now();

        loop {
            if self.state.get().is_exit() {
                break;
            }

            let now = Instant::now();
            let delta = now.duration_since(last_loop_instant);
            last_loop_instant = now;

            self.run_timers();

            let delta_milliseconds = delta.as_millis();

            if delta_milliseconds < 10 {
                sleep(Duration::from_millis(10) - delta)
            }
        }
    }

    /// Notify the run loop to break the loop and end.
    pub fn exit(&self) {
        self.state.set(State::Exit);
    }

    fn run_timers(&self) {
        let mut local_timers: Vec<Timer> = Vec::new();

        {
            let mut timers = self.timers.borrow_mut();

            for timer in timers.drain(..) {
                if timer.is_valid() {
                    local_timers.push(timer);
                }
            }

            timers.clear();
        }

        for timer in local_timers.iter() {
            if timer.fire_at() < Instant::now() {
                timer.fire();
            }
        }

        for timer in local_timers.drain(..) {
            self.add_timer(timer);
        }
    }
}

#[derive(Copy, Clone)]
enum State {
    Running,
    Exit
}

impl State {
    fn is_exit(&self) -> bool {
        match *self {
            State::Exit => true,
            _ => false,
        }
    }
}

use crate::ui::Timer;
use std::time::Instant;
use std::thread::sleep;
use std::cell::{Cell, RefCell};
use std::time::Duration;
use crate::singleton;

singleton::singleton!(
    RunLoop,
    timers: RefCell::new(Vec::new()),
    state: Cell::new(State::Running)
);

pub(crate) struct RunLoop {
    timers: RefCell<Vec<Timer>>,
    state: Cell<State>
}

impl RunLoop {
    pub fn add_timer(&self, timer: Timer) {
        let mut timers = self.timers.borrow_mut();
        timers.push(timer)
    }

    pub(crate) fn run(&self) {
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
    pub(crate) fn exit(&self) {
        self.state.set(State::Exit);
    }

    fn run_timers(&self) {
        let mut timers = self.timers.borrow_mut();
        timers.retain(|timer| {
            if timer.is_valid() {
                true
            } else {
                false
            }
        });

        for timer in timers.iter_mut() {
            if timer.fire_at() < Instant::now() {
                timer.fire();
            }
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

use crate::ui::timer::Timer;
use std::sync::{Mutex, RwLock};
use std::time::Instant;
use std::thread::sleep;
use std::time::Duration;
use crate::macros::*;
use crate::platform::thread;

singleton!(
    RunLoop,
    state: Mutex::new(State::Running),
    timers: RwLock::new(Vec::new())
);

pub struct RunLoop {
    state: Mutex<State>,
    timers: RwLock<Vec<Timer>>,
}

impl RunLoop {
    pub fn add_timer(&self, timer: Timer) {
        if !thread::is_main() {
            println!("Warning: attempted to add timer from non-main thread. The timer has not been added.");
            return;
        }
        let mut timers = self.timers.write().expect("Failed to lock timers for writing");
        timers.push(timer)
    }

    /// Run the run loop until the application exits.
    ///
    /// This isn't intended to be called in your app.
    pub fn run(&self) {
        let mut last_loop_instant = Instant::now();

        loop {
            let state = {
                let state = self.state.try_lock().expect("Failed to lock state for reading");
                *state
            };

            if state.is_exit() {
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
        let mut state = self.state.try_lock().expect("Failed to lock state for writing");
        *state = State::Exit;
    }

    fn run_timers(&self) {
        let mut local_timers: Vec<Timer> = Vec::new();

        {
            let mut timers = self.timers.write().expect("Failed to lock timers for writing");

            for timer in timers.drain(..) {
                if timer.is_valid() {
                    local_timers.push(timer);
                }
            }

            timers.clear();
        }

        for timer in local_timers.iter_mut() {
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

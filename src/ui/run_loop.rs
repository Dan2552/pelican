use crate::ui::Timer;
use std::time::Instant;
use std::thread::sleep;
use std::time::SystemTime;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

pub static mut MAIN: RunLoop = RunLoop {
    default_timers: Vec::new(),
};

pub(crate) struct RunLoop {
    default_timers: Vec<Timer>
}

pub enum Mode {
    Default
}

impl<'a> RunLoop {
    pub fn main() -> &'a mut RunLoop {
        &mut MAIN
    }

    pub fn add_timer(&mut self, timer: Timer, mode: Mode) {
        let mut default_timers = self.default_timers;

        match mode {
            Mode::Default => {
                default_timers.push(timer)
            }
        }
    }

    fn run(&self) {
        let mut last_loop_instant = Instant::now();

        loop {
            let now = Instant::now();
            let delta = now.duration_since(last_loop_instant);
            last_loop_instant = now;


            self.run_timers(Mode::Default);

            let delta_milliseconds = delta.as_millis();


            if delta_milliseconds < 10 {
                sleep(Duration::from_millis(10) - delta)
            }
        }
    }

    fn run_timers(&self, mode: Mode) {
        let mut timers = match mode {
            Mode::Default => self.default_timers
        };

        timers.retain(|timer| {
            if timer.is_valid() {
                true
            } else {
                false
            }
        });

        for timer in timers.iter_mut() {
            if timer.fire_at() > SystemTime::now() {
                timer.fire();
            }
        }
    }
}

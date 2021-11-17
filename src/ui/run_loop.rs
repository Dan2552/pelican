use crate::ui::Timer;
use std::time::Instant;
use std::thread::sleep;
use std::time::SystemTime;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use crate::singleton;

singleton::singleton!(RunLoop, timers: Vec::new());
pub(crate) struct RunLoop {
    timers: Vec<Timer>
}

pub enum Mode {
    Default
}

impl RunLoop {
    pub fn add_timer(&mut self, timer: Timer, mode: Mode) {
        self.timers.push(timer)
    }

    fn run(&mut self) {
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

    fn run_timers(&mut self, mode: Mode) {
        self.timers.retain(|timer| {
            if timer.is_valid() {
                true
            } else {
                false
            }
        });

        for timer in self.timers.iter_mut() {
            if timer.fire_at() > SystemTime::now() {
                timer.fire();
            }
        }
    }
}

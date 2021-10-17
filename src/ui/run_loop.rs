use crate::ui::Timer;
use std::time::Instant;
use std::thread::sleep;
use std::time::SystemTime;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

struct RunLoop {
    default_timers: Rc<RefCell<Vec<Timer>>>
}

enum Mode {
    Default
}

impl RunLoop {
    fn new() -> RunLoop {
        RunLoop {
            default_timers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn add_timer(&mut self, timer: Timer, mode: Mode) {
        let mut default_timers = self.default_timers.borrow_mut();

        match mode {
            Default => {
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
            Default => self.default_timers.borrow_mut()
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

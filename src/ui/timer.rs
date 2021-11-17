use std::time::SystemTime;
use std::time::Duration;

// A repeating or once-off Timer object, to be run by the main loop.
pub struct Timer {
    interval: Duration,
    repeats: bool,
    action: Box<dyn Fn() -> ()>,

    // If set to invalid, the timer will no longer run, and the main loop will
    // recognise it should be removed. In addition, an invalid timer cannot be
    // reused; it cannot be made valid again.
    is_valid: bool,

    // The last time the timer has fired. If it hasn't fired, it'll be the
    // time of initialization.
    last_fired_at: SystemTime,

    // Next target time to fire. If there is a delay to fire, or the fire takes
    // too long, it
    //
    // When invalidated, the last fire date.
    fire_at: SystemTime
}

impl Timer {
    pub fn new(interval: Duration, repeats: bool, action: impl Fn() -> () + 'static) -> Self {
        let now = SystemTime::now();
        Self {
            interval,
            repeats,
            action: Box::new(action),
            is_valid: true,
            last_fired_at: now,
            fire_at: now + interval
        }
    }

    pub fn new_once(action: impl Fn() -> () + 'static) -> Self {
        Timer::new(Duration::new(0, 0), false, action)
    }

    pub fn new_repeating(interval: Duration, action: impl Fn() -> () + 'static) -> Self {
        Timer::new(interval, true, action)
    }

    // Run the action
    pub(crate) fn fire(&mut self) {
        let current_fire_at = SystemTime::now();
        (self.action)();

        self.fire_at = current_fire_at + self.interval;
        self.last_fired_at = current_fire_at;

        if !self.repeats {
            self.is_valid = false
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn fire_at(&self) -> SystemTime {
        self.fire_at
    }

    // Sets the timer as invalidated. Meaning the run loop will recognise this:
    // * To not fire
    // * To be removed from the run loop
    pub fn invalidate(&mut self) {
        self.is_valid = false;
        self.fire_at = self.last_fired_at;
    }
}

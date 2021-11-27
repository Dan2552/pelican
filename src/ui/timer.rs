use std::time::Duration;
use std::time::Instant;
use std::cell::Cell;

// A repeating or once-off Timer object, to be run by the main loop.
pub struct Timer {
    interval: Duration,
    repeats: bool,
    action: Box<dyn Fn() -> ()>,

    // If set to invalid, the timer will no longer run, and the main loop will
    // recognise it should be removed. In addition, an invalid timer cannot be
    // reused; it cannot be made valid again.
    is_valid: Cell<bool>,

    // The last time the timer has fired. If it hasn't fired, it'll be the
    // time of initialization.
    last_fired_at: Cell<Instant>,

    // Next target time to fire. If there is a delay to fire, or the fire takes
    // too long, it
    //
    // When invalidated, the last fire date.
    fire_at: Cell<Instant>
}

impl Timer {
    pub fn new(interval: Duration, repeats: bool, action: impl Fn() -> () + 'static) -> Self {
        let now = Instant::now();
        Self {
            interval,
            repeats,
            action: Box::new(action),
            is_valid: Cell::new(true),
            last_fired_at: Cell::new(now),
            fire_at: Cell::new(now + interval)
        }
    }

    pub fn new_once(action: impl Fn() -> () + 'static) -> Self {
        Timer::new(Duration::new(0, 0), false, action)
    }

    pub fn new_repeating(interval: Duration, action: impl Fn() -> () + 'static) -> Self {
        Timer::new(interval, true, action)
    }

    // Run the action
    pub(crate) fn fire(&self) {
        let current_fire_at = Instant::now();
        (self.action)();

        self.fire_at.set(current_fire_at + self.interval);
        self.last_fired_at.set(current_fire_at);

        if !self.repeats {
            self.is_valid.set(false)
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid.get()
    }

    pub fn fire_at(&self) -> Instant {
        self.fire_at.get().clone()
    }

    // Sets the timer as invalidated. Meaning the run loop will recognise this:
    // * To not fire
    // * To be removed from the run loop
    pub fn invalidate(&self) {
        self.is_valid.set(false);
        self.fire_at.set(self.last_fired_at.get());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fire_at() {
        // by default
        //   it returns the next time to fire (now + given interval)
        let now = Instant::now();
        let interval = Duration::from_secs(360);
        let mut timer = Timer::new(interval, false, || {});

        let fire_at = timer.fire_at();

        assert!(fire_at > now, "fire_at should be higher than now");
        assert!(fire_at >= now + interval, "fire_at should be higher or equal to now+interval");

        // There's a wider gap here than expected intentionally, as extra time may have passed during the test execution
        assert!(fire_at < now + Duration::from_secs(361), "fire_at should be lower than too far beyond the interval");

        // when fired
        //   it refreshes the fire_at
        let now = Instant::now();
        timer.fire();
        assert_ne!(fire_at, timer.fire_at());

        let fire_at = timer.fire_at();

        assert!(fire_at > now, "after fire; fire_at should be higher than now");
        assert!(fire_at >= now + interval, "after fire; fire_at should be higher or equal to now+interval");

        // There's a wider gap here than expected intentionally, as extra time may have passed during the test execution
        assert!(fire_at < now + Duration::from_secs(361), "after fire; fire_at should be lower than too far beyond the interval");

        // when invalidated
        //   it sets the fire_at to the last time it fired
        let now = Instant::now();
        timer.invalidate();
        let fire_at_after_invalidate = timer.fire_at();

        assert_ne!(fire_at, fire_at_after_invalidate);
        assert!(fire_at_after_invalidate < now, "after invalidate; fire_at should be lower than now");
    }

    #[test]
    fn test_is_valid() {
        let interval = Duration::from_secs(360);
        let mut timer = Timer::new(interval, false, || {});

        assert!(timer.is_valid());
        timer.invalidate();
        assert!(!timer.is_valid());
    }

    static mut FIRED: bool = false;

    #[test]
    fn test_fire() {
        let interval = Duration::from_secs(360);

        let mut timer = Timer::new(interval, false, || {
            unsafe { FIRED = true; }
        });

        unsafe { assert!(!FIRED); }
        timer.fire();
        unsafe { assert!(FIRED); }
    }
}

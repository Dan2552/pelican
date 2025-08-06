use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use std::time::Instant;

// A repeating or once-off Timer object, to be run by the main loop.
pub struct Timer {
    inner: Arc<RwLock<TimerInner>>
}

struct TimerInner {
    interval: Duration,
    repeats: bool,
    action: Box<dyn Fn() + Send + Sync>,

    // If set to invalid, the timer will no longer run, and the main loop will
    // recognise it should be removed. In addition, an invalid timer cannot be
    // reused; it cannot be made valid again.
    is_valid: bool,

    // The last time the timer has fired. If it hasn't fired, it'll be the
    // time of initialization.
    last_fired_at: Instant,

    // Next target time to fire. If there is a delay to fire, or the fire takes
    // too long, it
    //
    // When invalidated, the last fire date.
    fire_at: Instant
}

impl Timer {
    pub fn new(interval: Duration, repeats: bool, action: impl Fn() + Send + Sync + 'static) -> Self {
        let now = Instant::now();
        Self {
            inner: Arc::new(RwLock::new(TimerInner {
                interval,
                repeats,
                action: Box::new(action),
                is_valid: true,
                last_fired_at: now,
                fire_at: now + interval
            }))
        }
    }

    pub fn new_once(action: impl Fn() + Send + Sync + 'static) -> Self {
        Timer::new(Duration::new(0, 0), false, action)
    }

    pub fn new_once_delayed(delay: Duration, action: impl Fn() + Send + Sync + 'static) -> Self {
        Timer::new(delay, false, action)
    }

    pub fn new_repeating(interval: Duration, action: impl Fn() + Send + Sync + 'static) -> Self {
        Timer::new(interval, true, action)
    }

    // Run the action
    pub(crate) fn fire(&mut self) {
        let current_fire_at = Instant::now();
        let mut inner = self.inner
            .write()
            .expect("Failed to write timer state");
        (inner.action)();

        inner.fire_at = current_fire_at + inner.interval;
        inner.last_fired_at = current_fire_at;

        if !inner.repeats {
            inner.is_valid = false;
        }
    }

    pub fn is_valid(&self) -> bool {
        self.inner
            .read()
            .expect("Failed to read timer state")
            .is_valid
    }

    pub fn fire_at(&self) -> Instant {
        self.inner
            .read()
            .expect("Failed to read timer state")
            .fire_at
    }

    // Sets the timer as invalidated. Meaning the run loop will recognise this:
    // * To not fire
    // * To be removed from the run loop
    pub fn invalidate(&mut self) {
        let mut inner = self.inner
            .write()
            .expect("Failed to write timer state");

        inner.is_valid = false;
        inner.fire_at = inner.last_fired_at.clone();
    }
}

impl Clone for Timer {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
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

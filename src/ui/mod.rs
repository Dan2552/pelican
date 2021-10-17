mod application_main;
pub(crate) use application_main::ApplicationMain;

mod color;
pub use color::Color;

mod event_loop;

mod run_loop;

mod timer;
use timer::Timer;

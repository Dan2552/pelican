use crate::graphics::Point;
use crate::ui::Touch;
use crate::macros::*;

singleton!(InputState, touches: Vec::new());

pub(crate) struct InputState {
    /// Any existing touches that are currently active. Used to keep track so
    /// we can monitor the whole lifecycle of any given touch.
    touches: Vec<Touch>
}

impl InputState {
    pub(crate) fn find_or_create_touch(&mut self, touch_id: i32) -> &mut Touch {
        let touch = self.touches.iter().find(|t| t.get_id() == touch_id);

        if touch.is_none() {
            let point = Point { x: 0, y: 0 };
            let touch = Touch::new(touch_id, point, crate::ui::touch::TouchPhase::Began);
            self.touches.push(touch);
        }

        let touch = self.touches.iter_mut().find(|t| t.get_id() == touch_id);
        touch.unwrap()
    }

    pub(crate) fn find_touch(&mut self, touch_id: i32) -> Option<&mut Touch> {
        self.touches.iter_mut().find(|t| t.get_id() == touch_id)
    }

    pub(crate) fn remove_touch(&mut self, touch_id: i32) {
        self.touches.retain(|t| t.get_id() != touch_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_or_create_touch() {
        let mut input_state = InputState { touches: Vec::new() };
        let touch_id = 1;

        let touch = input_state.find_or_create_touch(touch_id);
        assert_eq!(touch.get_id(), touch_id);
    }

    #[test]
    fn test_find_touch() {
        let mut input_state = InputState { touches: Vec::new() };
        let touch_id = 1;

        let touch = input_state.find_or_create_touch(touch_id);
        assert_eq!(touch.get_id(), touch_id);

        let touch = input_state.find_touch(touch_id);
        assert_eq!(touch.unwrap().get_id(), touch_id);
    }

    #[test]
    fn test_remove_touch() {
        let mut input_state = InputState { touches: Vec::new() };
        let touch_id = 1;

        let touch = input_state.find_or_create_touch(touch_id);
        assert_eq!(touch.get_id(), touch_id);

        input_state.remove_touch(touch_id);
        assert_eq!(input_state.find_touch(touch_id), None);
    }
}

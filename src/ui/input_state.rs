// use crate::graphics::Point;
// use crate::ui::Touch;
// use crate::macros::*;
// use crate::ui::event::TouchEvent;
// use std::time::Instant;

// singleton!(
//     InputState,
//     touch_event: TouchEvent { touches: Vec::new() }
// );

// pub(crate) struct InputState {
//     touch_event: TouchEvent
// }

// impl InputState {
//     pub(crate) fn touch_event(&self) -> &TouchEvent {
//         &self.touch_event
//     }

//     // pub(crate) fn find_or_create_touch(&mut self, touch_id: usize) -> &mut Touch {
//     //     let touches = self.touch_event.touches();
//     //     Event::Touch { touches } => {
//     //         let touch = touches.iter().find(|t| t.id() == touch_id);

//     //         if touch.is_none() {
//     //             let point = Point { x: 0, y: 0 };
//     //             let touch = Touch::new(
//     //                 touch_id,
//     //                 Instant::now(),
//     //                 point,
//     //                 crate::ui::touch::TouchPhase::Began
//     //             );
//     //             touches.push(touch);
//     //         }

//     //         let touch = touches.iter_mut().find(|t| t.id() == touch_id);
//     //         touch.unwrap()
//     //     }
//     // }

//     // pub(crate) fn find_touch(&mut self, touch_id: usize) -> Option<&mut Touch> {
//     //     match &mut self.touch_event {
//     //         Event::Touch { touches } => {
//     //             touches.iter_mut().find(|t| t.id() == touch_id)
//     //         }
//     //         _ => unreachable!()
//     //     }
//     // }

//     // pub(crate) fn remove_touch(&mut self, touch_id: usize) {
//     //     match &mut self.touch_event {
//     //         Event::Touch { touches } => {
//     //             touches.retain(|t| t.id() != touch_id);
//     //         }
//     //         _ => unreachable!()
//     //     }
//     // }
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_find_or_create_touch() {
// //         let mut input_state = InputState { touch_event: Event::Touch { touches: Vec::new() } };
// //         let touch_id = 1;

// //         let touch = input_state.find_or_create_touch(touch_id);
// //         assert_eq!(touch.id(), touch_id);
// //     }

// //     #[test]
// //     fn test_find_touch() {
// //         let mut input_state = InputState { touch_event: Event::Touch { touches: Vec::new() } };
// //         let touch_id = 1;

// //         let touch = input_state.find_or_create_touch(touch_id);
// //         assert_eq!(touch.id(), touch_id);

// //         let touch = input_state.find_touch(touch_id);
// //         assert_eq!(touch.unwrap().id(), touch_id);
// //     }

// //     #[test]
// //     fn test_remove_touch() {
// //         let mut input_state = InputState { touch_event: Event::Touch { touches: Vec::new() } };
// //         let touch_id = 1;

// //         let touch = input_state.find_or_create_touch(touch_id);
// //         assert_eq!(touch.id(), touch_id);

// //         input_state.remove_touch(touch_id);
// //         assert_eq!(input_state.find_touch(touch_id), None);
// //     }
// // }

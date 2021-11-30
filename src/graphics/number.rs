pub trait Number: Copy + std::fmt::Debug + PartialEq {}
impl Number for f32 {}
impl Number for i32 {}
impl Number for u32 {}

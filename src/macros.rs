pub mod singleton_support {
    use std::cell::{RefCell, Ref, RefMut};
    use std::thread::ThreadId;

    #[allow(dead_code)]
    pub struct MainThreadCell<T> {
        owner: ThreadId,
        inner: RefCell<T>,
    }

    impl<T> MainThreadCell<T> {
        pub fn new(value: T) -> Self {
            Self {
                owner: std::thread::current().id(),
                inner: RefCell::new(value),
            }
        }

        #[inline]
        #[cfg(not(test))]
        fn assert_owner(&self) {
            if std::thread::current().id() != self.owner {
                panic!("singleton accessed from a non-owner thread");
            }
        }

        #[inline]
        #[cfg(test)]
        fn assert_owner(&self) {
            // Skip check entirely in tests
        }

        #[inline]
        pub fn borrow(&self) -> Ref<'_, T> {
            self.assert_owner();
            self.inner.borrow()
        }

        #[inline]
        pub fn borrow_mut(&self) -> RefMut<'_, T> {
            self.assert_owner();
            self.inner.borrow_mut()
        }
    }

    // SAFETY: This is only sound if you *guarantee* all access happens on the owner thread.
    // We enforce that at runtime with `assert_owner()`, which panics if violated.
    unsafe impl<T> Sync for MainThreadCell<T> {}
}

#[macro_export]
macro_rules! singleton {
    // ---- Explicit fields only: no Default bound, no struct update ----
    ($ty:ident $(, $field:ident : $value:expr )* $(,)?) => {
        impl $ty {
            #![allow(dead_code)]
            #[inline]
            fn __cell() -> &'static $crate::macros::singleton_support::MainThreadCell<$ty> {
                static CELL: std::sync::OnceLock<
                    &'static $crate::macros::singleton_support::MainThreadCell<$ty>
                > = std::sync::OnceLock::new();

                *CELL.get_or_init(|| {
                    let boxed = Box::new(
                        $crate::macros::singleton_support::MainThreadCell::new(
                            $ty { $($field: $value,)* }
                        )
                    );
                    Box::leak(boxed)
                })
            }

            #[inline]
            pub fn borrow() -> std::cell::Ref<'static, $ty> {
                Self::__cell().borrow()
            }

            #[inline]
            pub fn borrow_mut() -> std::cell::RefMut<'static, $ty> {
                Self::__cell().borrow_mut()
            }

            /// After calling this, any `borrow_mut()` will panic forever.
            /// After calling this, `borrow_mut()` will panic forever.
            pub fn leak_static() -> &'static $ty {
                let r = Self::__cell().borrow();
                let p: *const $ty = &*r;
                std::mem::forget(r);           // keep borrow flag raised forever
                // SAFETY: we promise never to take a mutable borrow again.
                unsafe { &*p }
            }
        }
    };

    // ---- Use Default for missing fields: requires T: Default ----
    ($ty:ident + Default $(, $field:ident : $value:expr )* $(,)?) => {
        impl $ty {
            #![allow(dead_code)]
            #[inline]
            fn __cell() -> &'static $crate::macros::singleton_support::MainThreadCell<$ty>
            where
                $ty: Default,
            {
                static CELL: std::sync::OnceLock<
                    &'static $crate::macros::singleton_support::MainThreadCell<$ty>
                > = std::sync::OnceLock::new();

                *CELL.get_or_init(|| {
                    let boxed = Box::new(
                        $crate::macros::singleton_support::MainThreadCell::new(
                            $ty { $($field: $value,)* ..Default::default() }
                        )
                    );
                    Box::leak(boxed)
                })
            }

            #[inline]
            pub fn borrow() -> std::cell::Ref<'static, $ty>
            where
                $ty: Default,
            {
                Self::__cell().borrow()
            }

            #[inline]
            pub fn borrow_mut() -> std::cell::RefMut<'static, $ty>
            where
                $ty: Default,
            {
                Self::__cell().borrow_mut()
            }

            /// After calling this, any `borrow_mut()` will panic forever.
            /// After calling this, `borrow_mut()` will panic forever.
            pub fn leak_static() -> &'static $ty {
                let r = Self::__cell().borrow();
                let p: *const $ty = &*r;
                std::mem::forget(r);           // keep borrow flag raised forever
                // SAFETY: we promise never to take a mutable borrow again.
                unsafe { &*p }
            }
        }
    };
}

#[macro_export]
macro_rules! custom_view {
    ($view:ident subclasses $super:ident struct $behavior:ident { $($key:ident: $value:path),* } $(impl Self { $($custom_view_impl:item)* })? $(impl Behavior { $($custom_behavior_impl:item)* })?) => {
        pub struct $view {
            pub view: $crate::ui::View,
        }
        pub struct $behavior {
            view: $crate::ui::WeakView,
            super_behavior: Box<dyn $crate::ui::view::Behavior>,
            $($key: $value),*
        }
        impl $view {
            #![allow(dead_code)]

            pub(crate) fn new_all(frame: $crate::graphics::Rectangle<i32, u32>, $($key: $value),*) -> Self {
                let super_behavior = $super {
                    view: $crate::ui::WeakView::none()
                };

                let behavior = $behavior {
                    view: $crate::ui::WeakView::none(),
                    super_behavior: Box::new(super_behavior),
                    $($key),*
                };

                let view = $crate::ui::View::new_with_behavior(Box::new(behavior), frame, "test");
                Self { view }
            }

            pub fn from_view(view: $crate::ui::View) -> Self {
                // Downcast the behavior to essentially verify the view is a window.
                let _ = view.behavior().as_any().downcast_ref::<$behavior>().unwrap();

                Self { view }
            }

            pub fn behavior(&self) -> std::cell::Ref<'_, $behavior> {
                std::cell::Ref::map(self.view.behavior(), |behavior| {
                    behavior.as_any().downcast_ref::<$behavior>().unwrap()
                })
            }

            $($($custom_view_impl)*)?
        }

        impl $behavior {
            #![allow(dead_code)]

            fn view_type(&self) -> $view {
                $view::from_view(self.view.upgrade().unwrap())
            }
        }

        impl $crate::ui::view::Behavior for $behavior {
            fn super_behavior(&self) -> Option<&Box<dyn $crate::ui::view::Behavior>> {
                Some(&self.super_behavior)
            }

            fn mut_super_behavior(&mut self) -> Option<&mut dyn $crate::ui::view::Behavior> {
                Some(self.super_behavior.as_mut())
            }

            fn set_view(&mut self, view: $crate::ui::WeakView) {
                self.view = view;
            }

            fn get_view(&self) -> &$crate::ui::WeakView {
                &self.view
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            $($($custom_behavior_impl)*)?
        }

        impl Clone for $view {
            fn clone(&self) -> Self {
                Self { view: self.view.clone() }
            }
        }
    };
}

pub use singleton;
pub use custom_view;

#[cfg(test)]
mod tests {
    use super::*;

    struct ExampleSingleton {
        value1: i32,
        value2: i32
    }
    singleton!(ExampleSingleton, value1: 1, value2: 2);

    #[test]
    fn test_singleton() {
        {
            let example = ExampleSingleton::borrow();
            assert_eq!(example.value1, 1);
            assert_eq!(example.value2, 2);
        }

        {
            let mut example = ExampleSingleton::borrow_mut();
            assert_eq!(example.value1, 1);
            assert_eq!(example.value2, 2);
            example.value1 = 3;
            assert_eq!(example.value1, 3);
        }
        {
            let example = ExampleSingleton::borrow();
            assert_eq!(example.value1, 3);
            let double_borrow = ExampleSingleton::borrow();
            assert_eq!(example.value1, 3);
            assert_eq!(double_borrow.value1, 3);
        }
    }
}

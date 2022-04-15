#[macro_export]
macro_rules! singleton {
    ($singleton_name:ident$(, $key:ident: $value:expr)*) => {
        struct SingletonOwner {
            value: std::cell::RefCell<$singleton_name>,
        }
        impl SingletonOwner {
        }
        static mut SINGLETON_OWNER: SingletonOwner = SingletonOwner {
            value: std::cell::RefCell::new($singleton_name { $($key: $value),* }),
        };

        impl $singleton_name {
            #![allow(dead_code)]
            pub fn borrow<'a>() -> std::cell::Ref<'a, $singleton_name> {
                unsafe { SINGLETON_OWNER.value.borrow() }
            }

            pub fn borrow_mut<'a>() -> std::cell::RefMut<'a, $singleton_name> {
                unsafe { SINGLETON_OWNER.value.borrow_mut() }
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

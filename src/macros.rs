macro_rules! singleton {
    ($func_name:ident, $($key:ident: $value:expr),*) => {
        struct SingletonOwner {
            value: std::cell::RefCell<$func_name>,
        }
        impl SingletonOwner {
        }
        static mut SINGLETON_OWNER: SingletonOwner = SingletonOwner {
            value: std::cell::RefCell::new($func_name { $($key: $value),* }),
        };

        impl $func_name {
            pub fn borrow<'a>() -> std::cell::Ref<'a, $func_name> {
                unsafe { SINGLETON_OWNER.value.borrow() }
            }

            pub fn borrow_mut<'a>() -> std::cell::RefMut<'a, $func_name> {
                unsafe { SINGLETON_OWNER.value.borrow_mut() }
            }
        }
    };
}

macro_rules! custom_view {
    ($view:ident subclasses $super:ident struct $behavior:ident { $($key:ident: $value:path),* } $(view impl { $custom_view_impl:item })? behavior impl { $custom_behavior_impl:item }) => {
        pub struct $view {
            pub view: crate::ui::View,
        }
        pub struct $behavior {
            view: crate::ui::WeakView,
            super_behavior: Box<dyn Behavior>,
            $($key: $value),*
        }
        impl $view {
            pub fn new(frame: crate::graphics::Rectangle<i32, u32>, $($key: $value)*) -> Self {
                let super_behavior = $super {
                    view: crate::ui::WeakView::none()
                };
        
                let behavior = $behavior {
                    view: crate::ui::WeakView::none(),
                    super_behavior: Box::new(super_behavior),
                    $($key),*
                    // image: RefCell::new(image)
                };

                let view = crate::ui::View::new_with_behavior(Box::new(behavior), frame, "test");
                Self { view }
            }

            $($custom_view_impl)?
        }
        impl Behavior for $behavior {
            fn super_behavior(&self) -> Option<&Box<dyn Behavior>> {
                Some(&self.super_behavior)
            }
        
            fn mut_super_behavior(&mut self) -> Option<&mut dyn Behavior> {
                Some(self.super_behavior.as_mut())
            }
        
            fn set_view(&mut self, view: crate::ui::WeakView) {
                self.view = view;
            }
        
            fn get_view(&self) -> &crate::ui::WeakView {
                &self.view
            }
        
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            $custom_behavior_impl
        }
    };
}

macro_rules! super_behavior {
    ($self:ident) => {
        $self.super_behavior().unwrap();
    };
}

pub(crate) use singleton;
pub(crate) use custom_view;
pub(crate) use super_behavior;

#[cfg(test)]
mod tests {
    use super::*;

    struct ExampleSingleton {
        value1: i32,
        value2: i32
    }
    singleton!(ExampleSingleton, value1: 1, value2: 2);

    #[test]
    fn it_works() {
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

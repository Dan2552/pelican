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

pub(crate) use singleton;

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

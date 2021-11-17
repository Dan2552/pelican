macro_rules! singleton {
    ($func_name:ident, $($key:ident: $value:expr),*) => {
        use core::mem::replace;

        struct SingletonOwner {
            my_struct: Option<$func_name>,
        }
        impl SingletonOwner {
            fn adopt(&mut self) -> $func_name {
                let my_struct = replace(&mut self.my_struct, None);
                my_struct.unwrap()
            }

            fn disown(&mut self, my_struct: $func_name) {
                self.my_struct =  Some(my_struct);
            }
        }
        static mut SINGLETON_OWNER: SingletonOwner = SingletonOwner {
            my_struct: Some($func_name { $($key: $value),* }),
        };

        impl $func_name {
            pub fn adopt() -> $func_name {
                unsafe { SINGLETON_OWNER.adopt() }
            }

            pub fn disown(self) {
                unsafe { SINGLETON_OWNER.disown(self) };
            }
        }
    };
}

pub(crate) use singleton;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;

    struct ExampleSingleton {
        value1: i32,
        value2: i32
    }
    singleton!(ExampleSingleton, value1: 1, value2: 2);

    #[test]
    fn it_works() {
        let example = ExampleSingleton::adopt();
        assert_eq!(example.value1, 1);
        assert_eq!(example.value2, 2);
        example.disown();

        let mut example = ExampleSingleton::adopt();
        assert_eq!(example.value1, 1);
        assert_eq!(example.value2, 2);
        example.value1 = 3;
        assert_eq!(example.value1, 3);
        example.disown();

        let example = ExampleSingleton::adopt();
        assert_eq!(example.value1, 3);
        example.disown();
    }
}
